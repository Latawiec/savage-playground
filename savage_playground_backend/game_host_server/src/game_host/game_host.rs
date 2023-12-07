use std::path::{Path, PathBuf};

use crate::{
    instance::instance::Instance,
    room_server::{
        client::ClientID,
        message::{ClientMessage, ServerMessage},
        room::RoomHandle, self,
    },
};
use prost::Message;
use tokio::io::BufReader;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{ChildStderr, ChildStdin, ChildStdout},
    sync::broadcast,
    task::JoinHandle,
};

mod error {

    #[derive(Debug)]
    pub enum GamesMappingError {
        GameMappingFileError { reason: String },
        GameDirError { reason: String },
        FileContentIllFormed { resason: String },
        GameNotFound { reason: String },
    }

    #[derive(Debug)]
    pub enum GameAuthorithyError {
        NotAuthorized { reason: String },
    }

    #[derive(Debug)]
    pub enum GameInstanceError {
        Unknown { reason: String },
    }

    #[derive(Debug)]
    pub enum GameError {
        NoGameRunning,
        StdInClosed,
        StdOutClosed,
        StdErrClosed,
    }
}

pub struct GameHost {
    game_owner: ClientID,
    room_handle: RoomHandle,

    game_dir: PathBuf,
    game_runnable: PathBuf,
    game_instance: Option<Instance>,
}

struct ProtoStdin {
    stdin: ChildStdin,
}

impl ProtoStdin {
    pub async fn send<T: prost::Message>(&mut self, msg: &T) {
        let mut data = Vec::<u8>::new();
        msg.encode(&mut data).unwrap();
        let data_len = data.len();
        _ = self.stdin.write_u64(data_len as u64).await;
        _ = self.stdin.write(&data).await;
        _ = self.stdin.flush().await;
    }
}

struct ProtoStdout {
    stdout: BufReader<ChildStdout>,
}

impl ProtoStdout {
    pub async fn read<T: prost::Message + Default>(&mut self) -> Option<T> {
        let data_len = self.stdout.read_u64().await.unwrap();
        let mut data = Vec::<u8>::new();
        data.resize(data_len as usize, 0);         
        _ = self.stdout.read_exact(&mut data).await;

        let message = T::decode(data.as_slice());
        return Some(message.unwrap());
    }
}

struct ProtoStderr {
    stderr: BufReader<ChildStderr>,
}

impl ProtoStderr {
    pub async fn read<T: prost::Message + Default>(&mut self) -> Option<T> {
        let data_len = self.stderr.read_u64().await.unwrap();
        let mut data = Vec::<u8>::new();
        data.resize(data_len as usize, 0);         
        _ = self.stderr.read_exact(&mut data).await;

        let message = T::decode(data.as_slice());
        return Some(message.unwrap());
    }
}

impl GameHost {
    pub fn new(
        owner_id: ClientID,
        room_handle: RoomHandle,
        game_dir: PathBuf,
        game_runnable: PathBuf,
    ) -> GameHost {
        GameHost {
            game_owner: owner_id,
            room_handle,
            game_dir,
            game_runnable,
            game_instance: None,
        }
    }

    pub async fn run(&mut self) {
        let receiver = self.room_handle.receiver();
        let sender = self.room_handle.sender();

        let game_instance = Self::try_start_game(&self.game_dir, &self.game_runnable).await;
        if let Err(error) = &game_instance {
            tracing::error!("Failed starting game instance: {:?}", error);
            return;
        }
        let mut game_instance = game_instance.unwrap();

        let proto_stdin = ProtoStdin { stdin: game_instance.take_stdin().unwrap() };
        let proto_stdout = ProtoStdout { stdout: BufReader::new(game_instance.take_stdout().unwrap()) };
        let proto_stderr = ProtoStderr { stderr: BufReader::new(game_instance.take_stderr().unwrap()) };

        self.game_instance = Some(game_instance);
        Self::start_game_input_task(proto_stdin, receiver);
        Self::start_game_output_task(proto_stdout, sender, self.room_handle.room_id);
    }

    fn start_game_input_task(
        mut proto_pipe: ProtoStdin,
        mut client_msg_receiver: broadcast::Receiver<ClientMessage>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            const CLIENT_INPUT_PROTO_FILE: &str = "room_server_interface/proto/client_input.proto";
            const ENCODE_BUFFER_SIZE: usize = 8192; // 8kiB
            
            let mut encode_buffer = Vec::<u8>::new();
            encode_buffer.resize(ENCODE_BUFFER_SIZE, 0);
            while let Ok(msg) = client_msg_receiver.recv().await {
                match msg {
                    ClientMessage::Data { client_id, message } => {
                        match message {
                            crate::room_server::message::Message::Text { data } => todo!(),
                            crate::room_server::message::Message::Binary { data } => {
                                let client_game_message_proto = ::prost_types::Any::decode(data.as_slice());
                                if let Err(error) = &client_game_message_proto {
                                    tracing::error!("Couldn't decode client message: {:?}", error);
                                    return;
                                }
                                let client_game_message_proto = client_game_message_proto.unwrap();
                                let client_id = room_server_interface::proto::common::ClientId {
                                    value: client_id,
                                };
                                let client_input_proto = room_server_interface::proto::client_input::ClientInput {
                                    client_id: Some(client_id),
                                    game_input_message: Some(client_game_message_proto),
                                };
                                
                                encode_buffer.clear();
                                let client_input_encoded = client_input_proto.encode(&mut encode_buffer);
                                
                                if let Err(error) = &client_input_encoded {
                                    tracing::error!("Couldn't encode client message: {:?}", error);
                                    return;
                                }

                                // Build message forwarded to the instance:
                                let instance_input = room_server_interface::proto::instance_input::InstanceInput {
                                    instance_input_msg: Some(::prost_types::Any {
                                        type_url: CLIENT_INPUT_PROTO_FILE.to_owned(),
                                        value: encode_buffer.clone(),
                                    })
                                };
                                
                                proto_pipe.send(&instance_input);
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
    }

    fn start_game_output_task(
        mut proto_pipe: ProtoStdout,
        server_msg_sender: broadcast::Sender<ServerMessage>,
        room_id: u64,
    ) -> JoinHandle<()> {
        const CLIENT_OUPUT_PROTO_FILE: &str = "room_server_interface/proto/client_output.proto";
        const HOST_OUTPUT_PROTO_FILE: &str = "room_server_interface/proto/host_output.proto";
        const ENCODE_BUFFER_SIZE: usize = 8192; // 8kiB
            
        let mut encode_buffer = Vec::<u8>::new();
        encode_buffer.resize(ENCODE_BUFFER_SIZE, 0);
        tokio::spawn(async move {
            // This is very sketchy. What if stdin is already dead? I need to get rid of these interfaces I introduced. It only makes it harder.
            // I need to be able to distinguish None as "couldn't decode message" from pipe errors (e.g. pipe dead).
            while let Some(msg) = proto_pipe.read::<room_server_interface::proto::instance_output::InstanceOutput>().await {

                let instance_output_msg = msg.instance_output_msg.unwrap();

                match instance_output_msg.type_url.as_str() {
                    CLIENT_OUPUT_PROTO_FILE => {
                        let client_output_proto = room_server_interface::proto::client_output::ClientOutputBatch::decode(instance_output_msg.value.as_slice());
                        if let Err(error) = &client_output_proto {
                            tracing::error!("Couldn't decode instance message: {:?}", error);
                            return;
                        }
                        let client_output_proto = client_output_proto.unwrap();
                        if let Some(broadcast_proto) = &client_output_proto.broadcast {

                            encode_buffer.clear();
                            let _ = client_output_proto.broadcast.unwrap().client_output.unwrap().encode(&mut encode_buffer);

                            let room_broadcast = room_server::message::Message::Binary {
                                data: encode_buffer.as_slice().to_owned()
                            };
                            server_msg_sender.send(ServerMessage::Room { room_id, message:  room_broadcast } );

                            for direct_client_output in &client_output_proto.direct_messages {
                                encode_buffer.clear();
                                let client_id = direct_client_output.client_id.as_ref().unwrap().value;
                                let _ = direct_client_output.client_output.as_ref().unwrap().encode(&mut encode_buffer);
                                let direct_message = room_server::message::Message::Binary {
                                    data: encode_buffer.as_slice().to_owned()
                                };
                                server_msg_sender.send(ServerMessage::Client { client_id, message: direct_message });
                            }
                        }
                    }
                    HOST_OUTPUT_PROTO_FILE => {
                        let host_output_proto = room_server_interface::proto::host_output::HostOutput::decode(instance_output_msg.value.as_slice());
                        if let Err(error) = &host_output_proto {
                            tracing::error!("Couldn't decode instance message: {:?}", error);
                            return;
                        }
                        let host_output_proto = host_output_proto.unwrap();
                    }
                    _ => {

                    }
                }
            }
        })
    }

    async fn set_game_owner(&mut self, _client_id: u64, new_owner_id: u64) {
        self.game_owner = new_owner_id;
    }

    async fn try_start_game(
        game_dir: &Path,
        game_runnable: &Path,
    ) -> Result<Instance, error::GameInstanceError> {
        match Instance::new(&game_dir, &game_runnable) {
            Ok(instance) => Ok(instance),
            Err(error) => match error {
                crate::instance::instance::Error::StartupError { reason } => {
                    Err(error::GameInstanceError::Unknown { reason })
                }
                crate::instance::instance::Error::ProcessError { reason } => {
                    Err(error::GameInstanceError::Unknown { reason })
                }
            },
        }
    }
}
