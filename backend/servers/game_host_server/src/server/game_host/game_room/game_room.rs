use super::{connection::GameRoomConnectionHandle, disconnect_reason::GameRoomDisconnectReason};
use crate::{
    game_launcher::game_instance::{
        game_instance::GameInstance,
        proto_pipe::{ProtoStdin, ProtoStdout},
    },
    server::game_host::{handle_gen::HandleGenerator, types::ClientHandle},
};
use arc_swap::ArcSwap;
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
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use tokio::task::JoinHandle;

pub struct GameRoom {
    _game_room_config: GameConfig,
    _game_instance: GameInstance,

    client_input_task: JoinHandle<()>,
    game_output_task: JoinHandle<()>,

    client_handle_gen: HandleGenerator<ClientHandle>,
    client_input_sender: tokio::sync::mpsc::Sender<ClientInput>,

    shared_open_senders:
        Arc<ArcSwap<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<ClientOutput>>>>,
    lock_open_senders: Arc<Mutex<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<ClientOutput>>>>,

    closed: AtomicBool,
}

impl Drop for GameRoom {
    fn drop(&mut self) {
        self.client_input_task.abort();
        self.game_output_task.abort();
    }
}

impl GameRoom {
    const CLIENT_MESSAGE_RECEIVER_CAPACITY: usize = 1024;
    const CLIENT_MESSAGE_SENDER_CAPACITY: usize = 128;

    pub fn new(mut game_instance: GameInstance, game_room_config: GameConfig) -> GameRoom {
        let (client_input_sender, client_input_receiver) =
            tokio::sync::mpsc::channel(Self::CLIENT_MESSAGE_RECEIVER_CAPACITY);

        let shared_open_senders: Arc<
            ArcSwap<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<ClientOutput>>>,
        > = Default::default();

        let client_input_task = tokio::spawn(Self::client_input_task(
            client_input_receiver,
            game_instance.stdin.take().unwrap(),
        ));
        let game_output_task = tokio::spawn(Self::game_output_task(
            shared_open_senders.clone(),
            game_instance.stdout.take().unwrap(),
        ));

        GameRoom {
            _game_room_config: game_room_config,
            _game_instance: game_instance,
            client_input_task,
            game_output_task,
            client_handle_gen: Default::default(),
            client_input_sender,
            shared_open_senders,
            lock_open_senders: Default::default(),
            closed: AtomicBool::new(false),
        }
    }

    pub fn connect(&self, ws_stream: DuplexStream) -> GameRoomConnectionHandle {
        if self.closed.load(std::sync::atomic::Ordering::Relaxed) {
            return GameRoomConnectionHandle::new_closed(GameRoomDisconnectReason::RoomClosed);
        }

        let client_handle = self.client_handle_gen.next();
        let (output_sender, output_receiver) =
            tokio::sync::mpsc::channel::<ClientOutput>(Self::CLIENT_MESSAGE_SENDER_CAPACITY);
        let input_sender = self.client_input_sender.clone();

        let client_connection_handle =
            GameRoomConnectionHandle::new(client_handle, ws_stream, input_sender, output_receiver);

        {
            let mut lock = self.lock_open_senders.lock().unwrap();
            lock.insert(client_handle, output_sender);
            let new_senders = lock.clone();
            drop(lock);
            self.shared_open_senders.store(Arc::new(new_senders));
        }

        tokio::spawn({
            let client_connection_handle = client_connection_handle.clone();
            let lock_open_senders = self.lock_open_senders.clone();
            let shared_open_senders = self.shared_open_senders.clone();
            async move {
                client_connection_handle.wait().await;

                {
                    let mut lock = lock_open_senders.lock().unwrap();
                    lock.remove(&client_handle);
                    let new_senders = lock.clone();
                    drop(lock);
                    shared_open_senders.store(Arc::new(new_senders));
                }
            }
        });
        client_connection_handle
    }

    // private:
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
            println!("Sent");
            client_input_buffer.clear();
        }
    }

    async fn game_output_task(
        client_output_sender: Arc<
            ArcSwap<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<ClientOutput>>>,
        >,
        mut game_instance_output: ProtoStdout,
    ) {
        while let Some(game_output) = game_instance_output.read::<ClientOutputBatch>().await {
            println!("Received");
            let output_senders = client_output_sender.load();
            // Broadcast
            if let Some(broadcast_msg) = game_output.broadcast {
                let client_output = broadcast_msg.client_output;
                if let Some(client_output) = client_output {
                    for (_client_handle, sender) in output_senders.iter() {
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

                if let Some(sender) = output_senders.get(&ClientHandle(client_id.value)) {
                    let _ = sender.send(client_output).await;
                }
            }
        }
    }
}
