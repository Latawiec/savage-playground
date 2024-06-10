use super::{
    client_connection::ClientConnectionHandle,
    disconnect_reason::DisconnectReason,
    handle_gen::HandleGenerator,
    types::{ClientHandle, RoomHandle},
};
use crate::game_launcher::game_instance::{
    game_instance::GameInstance,
    proto_pipe::{ProtoStderr, ProtoStdin, ProtoStdout},
};
use rocket_ws::stream::DuplexStream;
use room_server_interface::{
    proto::{
        client_input::ClientInput,
        client_output::{ClientOutput, ClientOutputBatch},
    },
    schema::game_config::GameConfig,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, RwLock},
};

pub struct GameRoom {
    room_handle: RoomHandle,
    game_room_config: GameConfig,
    game_instance: GameInstance,
    client_handle_gen: HandleGenerator<ClientHandle>,
    client_input_sender: tokio::sync::mpsc::Sender<ClientInput>,
    client_input_receiver: tokio::sync::mpsc::Receiver<ClientInput>,
    client_output_senders: Arc<Mutex<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<ClientOutput>>>>,
    open_connections: Arc<Mutex<BTreeMap<ClientHandle, ClientConnectionHandle>>>,
}

impl GameRoom {
    const CLIENT_MESSAGE_RECEIVER_CAPACITY: usize = 1024;
    const CLIENT_MESSAGE_SENDER_CAPACITY: usize = 128;

    pub fn new(
        room_handle: RoomHandle,
        game_instance: GameInstance,
        game_room_config: GameConfig,
    ) -> GameRoom {
        let (client_input_sender, client_input_receiver) =
            tokio::sync::mpsc::channel(Self::CLIENT_MESSAGE_RECEIVER_CAPACITY);
        GameRoom {
            room_handle,
            game_room_config,
            game_instance,
            open_connections: Default::default(),
            client_handle_gen: Default::default(),
            client_input_sender,
            client_input_receiver,
            client_output_senders: Default::default(),
        }
    }

    pub fn connect(&self, ws_stream: DuplexStream) -> tokio::task::JoinHandle<DisconnectReason> {
        let client_handle = self.client_handle_gen.next();
        let (output_sender, output_receiver) =
            tokio::sync::mpsc::channel::<ClientOutput>(Self::CLIENT_MESSAGE_SENDER_CAPACITY);
        let input_sender = self.client_input_sender.clone();

        let client_connection_handle =
            ClientConnectionHandle::new(client_handle, ws_stream, input_sender, output_receiver);
        let client_close_notify = client_connection_handle.close_notify.clone();
        let client_close_reason = client_connection_handle.close_reason.clone();

        {
            let mut lock = self.client_output_senders.lock().unwrap();
            lock.insert(client_handle, output_sender);
        }
        {
            let mut lock = self.open_connections.lock().unwrap();
            lock.insert(client_handle, client_connection_handle);
        }


        let open_connections = self.open_connections.clone();
        let client_output_senders = self.client_output_senders.clone();
        tokio::spawn(async move {
            // Wait for connection to finish...
            client_close_notify.notified().await;

            {
                let mut lock = open_connections.lock().unwrap();
                lock.remove(&client_handle);
            }
            {
                let mut lock = client_output_senders.lock().unwrap();
                lock.remove(&client_handle);
            }

            client_close_reason
                .get()
                .expect("Closed without a reason???")
                .clone()
        })
    }

    async fn client_input_task(
        mut client_input_receiver: tokio::sync::mpsc::Receiver<ClientInput>,
        mut game_instance_input: ProtoStdin,
    ) {
        const CLIENT_INPUT_BUFFER_CAPACITY: usize = 256;
        let mut client_input_buffer =
            Vec::<ClientInput>::with_capacity(CLIENT_INPUT_BUFFER_CAPACITY);

        loop {
            if client_input_receiver
                .recv_many(&mut client_input_buffer, CLIENT_INPUT_BUFFER_CAPACITY)
                .await
                == 0
            {
                // Channel closed...
                break;
            }
            game_instance_input.send_many(&client_input_buffer).await;
            client_input_buffer.clear();
        }
    }

    async fn game_output_task(
        client_output_senders: RwLock<
            BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<ClientOutput>>,
        >,
        mut game_instance_output: ProtoStdout,
    ) {
        while let Some(game_output) = game_instance_output.read::<ClientOutputBatch>().await {
            // Broadcast
            if let Some(broadcast_msg) = game_output.broadcast {
                let client_output = broadcast_msg.client_output;
                if let Some(client_output) = client_output {
                    let client_output_senders = client_output_senders.read().unwrap();
                    for (_client_handle, sender) in client_output_senders.iter() {
                        let _ = sender.send(client_output.clone()).await;
                    }
                } else {
                    tracing::warn!("Got empty broadcast message. Unexpected.");
                }
            }

            // Direct messages
            for direct_message in game_output.direct_messages {
                let client_id = direct_message.client_id;
                let client_output = direct_message.client_output;

                if client_id.is_none() {
                    tracing::warn!("Direct message without client_id.");
                    continue;
                }
                if client_output.is_none() {
                    tracing::warn!("Direct message without body");
                    continue;
                }

                let client_id = client_id.unwrap();
                let client_output = client_output.unwrap();

                let client_output_senders = client_output_senders.read().unwrap();
                if let Some(sender) = client_output_senders.get(&ClientHandle(client_id.value)) {
                    let _ = sender.send(client_output).await;
                }
            }
        }
    }

    async fn game_error_task(mut game_instance_error: ProtoStderr) {
        // ???
    }
}
