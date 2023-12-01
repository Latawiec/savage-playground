use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    instance::instance::Instance,
    room_server::{
        client::ClientID,
        message::{ClientMessage, ServerMessage},
        room::RoomHandle,
    },
};
use async_trait::async_trait;
use host_management_interface::proto;
use host_runtime_interface::interface::host_interface::HostInstanceInterface;
use prost::Message;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{ChildStderr, ChildStdin, ChildStdout},
    sync::broadcast,
    task::JoinHandle,
};

use super::message::{self, Request};

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

struct ProtoPipe {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    stderr: BufReader<ChildStderr>,
}

#[async_trait]
impl HostInstanceInterface for ProtoPipe {
    async fn send(&self, msg: &host_runtime_interface::proto::host_instance::ClientMessage) {
        let data = msg.encode_to_vec();
        let data_len: u64 = data.len() as u64;

        self.stdin.write_u64(data_len).await;
        self.stdin.write(&data).await;
        self.stdin.flush().await;
    }

    async fn read(&self) -> Option<host_runtime_interface::proto::host_instance::InstanceMessage> {
        let data_len: u64 = self.stdout.read_u64().await.unwrap();
        let data = Vec::<u8>::new();
        data.resize(data_len as usize, 0);
        self.stdout.read_exact(&mut data).await.unwrap();

        let buffer = prost::bytes::Bytes::from(data);

        let message = host_runtime_interface::proto::host_instance::InstanceMessage::decode(buffer);
        if let Err(error) = &message {
            tracing::error!("Failed to decode message: {}", error);
            return None;
        }

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
        let proto_pipe = Arc::new(ProtoPipe {
            stdin: game_instance.take_stdin().unwrap(),
            stdout: BufReader::new(game_instance.take_stdout().unwrap()),
            stderr: BufReader::new(game_instance.take_stderr().unwrap()),
        });

        self.game_instance = Some(game_instance);
        Self::start_game_input_task(proto_pipe.clone(), receiver);
        Self::start_game_output_task(proto_pipe.clone(), sender, self.room_handle.room_id);
    }

    fn start_game_input_task(
        proto_pipe: Arc<ProtoPipe>,
        client_msg_receiver: broadcast::Receiver<ClientMessage>,
    ) -> JoinHandle<()> {
        tokio::spawn(async {
            while let Ok(msg) = client_msg_receiver.recv().await {
                match msg {
                    ClientMessage::Data { client_id, message } => {
                        match message {
                            crate::room_server::message::Message::Text { data } => todo!(),
                            crate::room_server::message::Message::Binary { data } => {
                                let buffer = prost::bytes::Bytes::from(data);
                                let proto_client_message = host_runtime_interface::proto::host_client::ClientMessage::decode(buffer);

                                if let Err(error) = &proto_client_message {
                                    tracing::error!("Couldn't decode client message: {:?}", error);
                                }
                                let proto_client_message = proto_client_message.unwrap();

                                // Build message forwarded to the instance:
                                let proto_client_id =
                                    host_runtime_interface::proto::host_instance::ClientId {
                                        value: client_id,
                                    };
                                let proto_instance_message =
                                    host_runtime_interface::proto::host_instance::ClientMessage {
                                        client_id: Some(proto_client_id),
                                        game_input_message: proto_client_message.game_input_message,
                                    };

                                proto_pipe.send(&proto_instance_message);
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
    }

    fn start_game_output_task(
        proto_pipe: Arc<ProtoPipe>,
        server_msg_sender: broadcast::Sender<ServerMessage>,
        room_id: u64,
    ) -> JoinHandle<()> {
        tokio::spawn(async {
            // This is very sketchy. What if stdin is already dead? I need to get rid of these interfaces I introduced. It only makes it harder.
            // I need to be able to distinguish None as "couldn't decode message" from pipe errors (e.g. pipe dead).
            loop {
                if let Some(msg) = proto_pipe.read().await {
                    for direct_msg in &msg.direct_messages {
                        let message = ServerMessage::Client {
                            client_id: direct_msg.client_id.unwrap().value,
                            message: crate::room_server::message::Message::Binary {
                                data: direct_msg.game_output_message.unwrap().encode_to_vec(),
                            },
                        };
                        server_msg_sender.send(message);
                    }

                    if let Some(msg) = &msg.broadcast {
                        let message = ServerMessage::Room {
                            room_id: room_id,
                            message: crate::room_server::message::Message::Binary {
                                data: msg.game_output_message.unwrap().encode_to_vec(),
                            },
                        };
                        server_msg_sender.send(message);
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
