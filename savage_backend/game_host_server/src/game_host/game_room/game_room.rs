use crate::{game_host::{handle_gen::HandleGenerator, interface::schema::game_config::GameConfig, types::ClientHandle}, game_launcher::game_instance::{game_instance::GameInstance, proto_pipe::{ProtoStderr, ProtoStdin, ProtoStdout}}};

use super::{connection::GameRoomConnectionHandle, disconnect_reason::GameRoomDisconnectReason};
use std::{
    collections::BTreeMap,
    sync::{atomic::{AtomicBool, AtomicU64}, Arc, Mutex},
};
use arc_swap::ArcSwap;
use game_interface::proto::{game_input::{ClientInput, GameInput, RoomInput}, game_output::{GameMessage, GameOutput}};
use rocket_ws::stream::DuplexStream;
use tokio::{io::AsyncReadExt, task::JoinHandle, time::Instant};
use tracing::{error, info_span, trace, warn, Instrument};

#[derive(Default)]
struct GameRoomSettings {
    pub game_master_id: Mutex<Option<ClientHandle>>,
}

const ROOM_SIZE_HARD_LIMIT: u64 = 24;

#[derive(Clone)]
struct GameConnectionData {
    message_sender: tokio::sync::mpsc::Sender<GameMessage>,
    connection_handle: GameRoomConnectionHandle,
}

pub struct GameRoom {
    _game_room_config: GameConfig,
    _game_instance: GameInstance,

    client_input_task: JoinHandle<()>,
    game_output_task: JoinHandle<()>,
    game_error_task: JoinHandle<()>,

    client_handle_gen: HandleGenerator<ClientHandle>,
    client_input_sender: tokio::sync::mpsc::Sender<ClientInput>,
    room_input_sender: tokio::sync::mpsc::Sender<RoomInput>,

    room_settings: Arc<GameRoomSettings>,

    shared_open_senders:
        Arc<ArcSwap<BTreeMap<ClientHandle, GameConnectionData>>>,
    lock_open_senders: Arc<Mutex<BTreeMap<ClientHandle, GameConnectionData>>>,
    players_counter: AtomicU64,

    closed: AtomicBool,
    span: tracing::span::Span,
    created: Instant,
}

impl Drop for GameRoom {
    fn drop(&mut self) {
        self.closed.store(true, std::sync::atomic::Ordering::Relaxed);
        self.client_input_task.abort();
        self.game_output_task.abort();
        self.game_error_task.abort();

        for (_, client_data) in self.lock_open_senders.lock().expect("Couldn't lock a mutex").iter() {
            client_data.connection_handle.close(GameRoomDisconnectReason::RoomClosed);
        }

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
    const ROOM_MESSAGE_SENDER_CAPACITY: usize = 128;

    pub fn new(mut game_instance: GameInstance, game_room_config: GameConfig, game_room_span: tracing::Span) -> GameRoom {
        let (client_input_sender, client_input_receiver) =
            tokio::sync::mpsc::channel(Self::CLIENT_MESSAGE_RECEIVER_CAPACITY);
        let (room_input_sender, room_input_receiver) = 
            tokio::sync::mpsc::channel(Self::ROOM_MESSAGE_SENDER_CAPACITY);

        let shared_open_senders: Arc<
            ArcSwap<BTreeMap<ClientHandle, GameConnectionData>>,
        > = Default::default();

        let client_input_task = tokio::spawn(Self::game_input_task(
            client_input_receiver,
            room_input_receiver,
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
            room_input_sender,
            room_settings: Default::default(),
            shared_open_senders,
            lock_open_senders: Default::default(),
            players_counter: Default::default(),
            closed: AtomicBool::new(false),
            span: game_room_span,
            created: Instant::now(),
        }
    }

    pub fn connect(&self, ws_stream: DuplexStream) -> GameRoomConnectionHandle {
        if self.closed.load(std::sync::atomic::Ordering::Relaxed) {
            return GameRoomConnectionHandle::new_closed(GameRoomDisconnectReason::RoomClosed);
        }

        if self.players_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= ROOM_SIZE_HARD_LIMIT {
            self.players_counter.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            return GameRoomConnectionHandle::new_closed(GameRoomDisconnectReason::RoomFull);
        }

        let client_handle = self.client_handle_gen.next();
        let (output_sender, output_receiver) =
            tokio::sync::mpsc::channel::<GameMessage>(Self::CLIENT_MESSAGE_SENDER_CAPACITY);
        let input_sender = self.client_input_sender.clone();

        let client_connection_handle =
            GameRoomConnectionHandle::new(
                client_handle,
                ws_stream,
                input_sender,
                output_receiver,
                info_span!(parent: &self.span, "game_room_connection")
            );


            
        let mut room_message = RoomInput::default();
        {
            let mut lock = self.lock_open_senders.lock().unwrap();
            lock.insert(client_handle, GameConnectionData {
                message_sender: output_sender,
                connection_handle: client_connection_handle.clone(),
            });
            let new_senders = lock.clone();
            drop(lock);
            self.shared_open_senders.store(Arc::new(new_senders));
            room_message.players_joined.push(client_handle.0);
        }

        { // Set game-master
            let mut game_master_id_lock = self.room_settings.game_master_id.lock().expect("Couldn't lock a mutex");
            if game_master_id_lock.is_none() {
                game_master_id_lock.replace(client_handle);
                room_message.game_master_id = Some(client_handle.0);
            }
        }

        tokio::spawn({
            let room_settings = self.room_settings.clone();
            let client_connection_handle = client_connection_handle.clone();
            let lock_open_senders = self.lock_open_senders.clone();
            let shared_open_senders = self.shared_open_senders.clone();
            let room_input_sender = self.room_input_sender.clone();
            
            async move {
                if let Err(error) = room_input_sender.send(room_message).await {
                    error!("couldn't send initial room input message: {}", error);
                }
                client_connection_handle.wait().await;

                // Client is leaving...

                let mut room_message = RoomInput::default();
                {
                    let mut lock = lock_open_senders.lock().unwrap();
                    lock.remove(&client_handle);
                    room_message.players_left.push(client_handle.0);

                    { // Set game-master
                        let mut game_master_id_lock = room_settings.game_master_id.lock().expect("Couldn't lock a mutex");
                        if let Some(game_master_handle) = &*game_master_id_lock {
                            if game_master_handle.0 == client_handle.0 {
                                if let Some(other_player_handle) = lock.first_entry() {
                                    game_master_id_lock.replace(other_player_handle.key().clone());
                                    room_message.game_master_id = Some(other_player_handle.key().0);
                                } else {
                                    game_master_id_lock.take();
                                    room_message.game_master_id = Some(0);
                                }
                            }
                        }
                    }

                    let new_senders = lock.clone();
                    drop(lock);
                    shared_open_senders.store(Arc::new(new_senders));
                }

                if let Err(error) = room_input_sender.send(room_message).await {
                    // It's just a warning. It's possible that the room simply doesn't exist anymore.
                    warn!("couldn't send final room input message: {}", error);
                }
            }
        });
        client_connection_handle
    }

    // private:
    async fn game_input_task(
        mut client_input_receiver: tokio::sync::mpsc::Receiver<ClientInput>,
        mut room_input_receiver: tokio::sync::mpsc::Receiver<RoomInput>,
        mut game_instance_input: ProtoStdin,
    ) {
        const CLIENT_INPUT_BUFFER_CAPACITY: usize = 64;
        const ROOM_INPUT_BUFFER_CAPACITY: usize = 32;
        let mut client_input_buffer =
            Vec::<ClientInput>::with_capacity(CLIENT_INPUT_BUFFER_CAPACITY);
        let mut room_input_buffer =
            Vec::<RoomInput>::with_capacity(ROOM_INPUT_BUFFER_CAPACITY);
        loop {
            tokio::select! {
                client_input_bytes = client_input_receiver.recv_many(&mut client_input_buffer, CLIENT_INPUT_BUFFER_CAPACITY) => {
                    if client_input_bytes == 0
                    {
                        // Channel closed...
                        error!("client input channel has been closed");
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
                },
                room_input_bytes = room_input_receiver.recv_many(&mut room_input_buffer, ROOM_INPUT_BUFFER_CAPACITY) => {
                    if room_input_bytes == 0
                    {
                        error!("room input channel has been closed");
                        break;
                    }
                    trace!(messages_coutn = room_input_buffer.len(), "room input");
                    for room_input in room_input_buffer.drain(..) {
                        let game_input_proto = GameInput {
                            client_input: None,
                            room_input: Some(room_input)
                        };
                        game_instance_input.send(&game_input_proto).await;
                    }
                    room_input_buffer.clear();
                }
            };
        }
    }

    async fn game_output_task(
        client_connections: Arc<
            ArcSwap<BTreeMap<ClientHandle, GameConnectionData>>,
        >,
        mut game_instance_output: ProtoStdout,
    ) {
        while let Some(game_output) = game_instance_output.read::<GameOutput>().await {
            let client_connections = client_connections.load();
            // Broadcast
            trace!(clients_count = client_connections.len(), "game output broadcast");
            if let Some(broadcast_msg) = game_output.broadcast {
                for (_client_handle, client_data) in client_connections.iter() {
                    let _ = client_data.message_sender.send(broadcast_msg.clone()).await;
                }
            }

            // Direct messages
            trace!(messages_count = client_connections.len(), "game output direct message");
            for direct_message in game_output.direct_messages {
                let client_id = direct_message.receiver_id;
                let message = direct_message.game_output;

                if message.is_none() {
                    warn!("Direct message without body");
                    continue;
                }
                let message = message.unwrap();

                if let Some(client_data) = client_connections.get(&ClientHandle(client_id)) {
                    let _ = client_data.message_sender.send(message).await;
                }
            }
        }
    }

    async fn game_error_task(
        mut game_instance_error: ProtoStderr,
    ) {
        let mut string_buf = String::new();
        while let Ok(size) = game_instance_error.raw_reader().read_to_string(&mut string_buf).await {
            if size != 0 {
                error!("game error: {}", string_buf);
                string_buf.clear();
            }
        }
    }
}
