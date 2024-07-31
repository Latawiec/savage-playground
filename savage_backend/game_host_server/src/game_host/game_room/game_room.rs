use crate::{game_host::{handle_gen::HandleGenerator, interface::schema::game_config::GameConfig, types::ClientHandle}, game_launcher::game_instance::{game_instance::GameInstance, proto_pipe::{ProtoStderr, ProtoStdin, ProtoStdout}}};

use super::{connection::GameRoomConnectionHandle, disconnect_reason::GameRoomDisconnectReason};
use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use arc_swap::ArcSwap;
use game_interface::proto::{game_input::{ClientInput, GameInput}, game_output::{GameMessage, GameOutput}};
use rocket_ws::stream::DuplexStream;
use tokio::{io::AsyncReadExt, task::JoinHandle, time::Instant};
use tracing::{trace, trace_span, warn, error, Instrument};

pub struct GameRoom {
    _game_room_config: GameConfig,
    _game_instance: GameInstance,

    client_input_task: JoinHandle<()>,
    game_output_task: JoinHandle<()>,
    game_error_task: JoinHandle<()>,

    client_handle_gen: HandleGenerator<ClientHandle>,
    client_input_sender: tokio::sync::mpsc::Sender<ClientInput>,

    shared_open_senders:
        Arc<ArcSwap<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<GameMessage>>>>,
    lock_open_senders: Arc<Mutex<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<GameMessage>>>>,

    closed: AtomicBool,
    span: tracing::span::Span,
    created: Instant,
}

impl Drop for GameRoom {
    fn drop(&mut self) {
        self.client_input_task.abort();
        self.game_output_task.abort();
        self.game_error_task.abort();
        self.span.in_scope(|| {
            let destroy_time = Instant::now();
            let lifetime = destroy_time.duration_since(self.created);
            trace!(lifetime_ms = lifetime.as_millis(), "destroyed");
        });
    }
}

impl GameRoom {
    const CLIENT_MESSAGE_RECEIVER_CAPACITY: usize = 1024;
    const CLIENT_MESSAGE_SENDER_CAPACITY: usize = 128;

    pub fn new(mut game_instance: GameInstance, game_room_config: GameConfig) -> GameRoom {
        let (client_input_sender, client_input_receiver) =
            tokio::sync::mpsc::channel(Self::CLIENT_MESSAGE_RECEIVER_CAPACITY);

        let shared_open_senders: Arc<
            ArcSwap<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<GameMessage>>>,
        > = Default::default();

        let game_id = game_room_config.game_id.clone().unwrap_or("UNKNOWN".to_owned());
        let game_room_span = trace_span!("game_room", game_id, "lifetime");

        let client_input_task = tokio::spawn(Self::client_input_task(
            client_input_receiver,
            game_instance.stdin.take().unwrap(),
        ).instrument(game_room_span.clone()));
        let game_output_task = tokio::spawn(Self::game_output_task(
            shared_open_senders.clone(),
            game_instance.stdout.take().unwrap(),
        ).instrument(game_room_span.clone()));
        let game_error_task = tokio::spawn(Self::game_error_task(
            game_instance.stderr.take().unwrap(),
        ).instrument(game_room_span.clone()));

        game_room_span.in_scope(|| {
            trace!("created");
        });

        GameRoom {
            _game_room_config: game_room_config,
            _game_instance: game_instance,
            client_input_task,
            game_output_task,
            game_error_task,
            client_handle_gen: Default::default(),
            client_input_sender,
            shared_open_senders,
            lock_open_senders: Default::default(),
            closed: AtomicBool::new(false),
            span: game_room_span,
            created: Instant::now(),
        }
    }

    pub fn connect(&self, ws_stream: DuplexStream) -> GameRoomConnectionHandle {
        if self.closed.load(std::sync::atomic::Ordering::Relaxed) {
            return GameRoomConnectionHandle::new_closed(GameRoomDisconnectReason::RoomClosed);
        }

        let client_handle = self.client_handle_gen.next();
        let (output_sender, output_receiver) =
            tokio::sync::mpsc::channel::<GameMessage>(Self::CLIENT_MESSAGE_SENDER_CAPACITY);
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
                trace!("input channel has been closed");
                break;
            }
            trace!(messages_count = client_input_buffer.len(), "client input");
            for client_input in client_input_buffer.drain(..) {
                let game_input_proto = GameInput {
                    client_input: Some(client_input),
                    room_input: None
                };
                game_instance_input.send(&game_input_proto).await;
            }
            client_input_buffer.clear();
        }
    }

    async fn game_output_task(
        client_output_sender: Arc<
            ArcSwap<BTreeMap<ClientHandle, tokio::sync::mpsc::Sender<GameMessage>>>,
        >,
        mut game_instance_output: ProtoStdout,
    ) {
        while let Some(game_output) = game_instance_output.read::<GameOutput>().await {
            let output_senders = client_output_sender.load();
            // Broadcast
            trace!(clients_count = output_senders.len(), "game output broadcast");
            if let Some(broadcast_msg) = game_output.broadcast {
                for (_client_handle, sender) in output_senders.iter() {
                    let _ = sender.send(broadcast_msg.clone()).await;
                }
            }

            // Direct messages
            trace!(messages_count = output_senders.len(), "game output direct message");
            for direct_message in game_output.direct_messages {
                let client_id = direct_message.receiver_id;
                let message = direct_message.game_output;

                if message.is_none() {
                    warn!("Direct message without body");
                    continue;
                }
                let message = message.unwrap();

                if let Some(sender) = output_senders.get(&ClientHandle(client_id)) {
                    let _ = sender.send(message).await;
                }
            }
        }
    }

    async fn game_error_task(
        mut game_instance_error: ProtoStderr,
    ) {
        let mut string_buf = String::new();
        while let Ok(_size) = game_instance_error.raw_reader().read_to_string(&mut string_buf).await {
            error!("{}", string_buf);
            string_buf.clear();
        }
    }
}
