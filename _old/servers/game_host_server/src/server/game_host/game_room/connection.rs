use std::sync::Arc;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use prost::Message;
use rocket_ws::stream::DuplexStream;
use room_server_interface::proto::{
    client_input::ClientInput, client_output::ClientOutput, common::ClientId,
};
use tokio::task::JoinHandle;

use crate::server::game_host::types::ClientHandle;

use super::disconnect_reason::GameRoomDisconnectReason;

struct OnceNotify<T: Clone> {
    notify: tokio::sync::Notify,
    value: tokio::sync::OnceCell<T>,
}

impl<T: Clone> OnceNotify<T> {
    pub async fn notified(&self) -> T {
        let notified_fut = self.notify.notified();
        if let Some(value) = self.value.get() {
            return value.clone();
        }

        notified_fut.await;
        self.value
            .get()
            .expect("Notified without result set.")
            .clone()
    }

    pub fn notify(&self, value: T) {
        let _ = self.value.set(value);
        self.notify.notify_waiters();
    }
}

impl<T: Clone> Default for OnceNotify<T> {
    fn default() -> Self {
        Self {
            notify: Default::default(),
            value: Default::default(),
        }
    }
}

#[derive(Clone, Default)]
pub struct GameRoomConnectionHandle {
    close_notify: Arc<OnceNotify<GameRoomDisconnectReason>>,
    _actor: Arc<Option<GameRoomConnectionActor>>,
}

impl GameRoomConnectionHandle {
    pub fn new(
        client_handle: ClientHandle,
        client_connection: DuplexStream,
        client_message_tx: tokio::sync::mpsc::Sender<ClientInput>,
        client_message_rx: tokio::sync::mpsc::Receiver<ClientOutput>,
    ) -> GameRoomConnectionHandle {
        let (rx, tx) = client_connection.split();
        let close_notify = Arc::new(OnceNotify::<GameRoomDisconnectReason>::default());

        let client_reader_task = tokio::spawn({
            let close_notify = close_notify.clone();
            async move {
                let close_reason = GameRoomConnectionActor::client_reader_task(
                    client_handle.clone(),
                    tx,
                    client_message_tx,
                )
                .await;
                close_notify.notify(close_reason);
            }
        });

        let client_sender_task = tokio::spawn({
            let close_notify = close_notify.clone();
            async move {
                let close_reason = GameRoomConnectionActor::client_writer_task(
                    client_handle.clone(),
                    rx,
                    client_message_rx,
                )
                .await;
                close_notify.notify(close_reason);
            }
        });

        let _actor = Arc::new(Some(GameRoomConnectionActor {
            client_reader_task,
            client_sender_task,
        }));

        GameRoomConnectionHandle {
            close_notify,
            _actor,
        }
    }

    pub fn new_closed(close_reason: GameRoomDisconnectReason) -> GameRoomConnectionHandle {
        let close_notify = Arc::new(OnceNotify::<GameRoomDisconnectReason>::default());
        close_notify.notify(close_reason);
        GameRoomConnectionHandle {
            close_notify,
            _actor: Default::default(),
        }
    }

    pub async fn wait(&self) -> GameRoomDisconnectReason {
        self.close_notify.notified().await
    }

    fn close(&mut self, reason: GameRoomDisconnectReason) {
        self.close_notify.notify(reason);
    }
}

struct GameRoomConnectionActor {
    client_reader_task: JoinHandle<()>,
    client_sender_task: JoinHandle<()>,
}

impl Drop for GameRoomConnectionActor {
    fn drop(&mut self) {
        self.client_reader_task.abort();
        self.client_sender_task.abort();
    }
}

/***
 * Handles reading and writing of data sent by the connected client.
 */
impl GameRoomConnectionActor {
    /***
     * Reads messages from the GameRoom
     * Writes to the Client
     */
    async fn client_writer_task(
        _client_handle: ClientHandle,
        mut client_message_rx: SplitSink<DuplexStream, rocket_ws::Message>,
        mut game_to_client_receiver: tokio::sync::mpsc::Receiver<ClientOutput>,
    ) -> GameRoomDisconnectReason {
        let mut encoding_buffer = Vec::<u8>::new();
        while let Some(msg) = game_to_client_receiver.recv().await {
            if let None = &msg.game_output_message {
                tracing::error!("Ill formed message - game output part missing");
                continue;
            }

            if let Err(error) = msg
                .game_output_message
                .unwrap()
                .encode(&mut encoding_buffer)
            {
                tracing::error!("Message encoding error: {}", error);
                continue;
            }

            if let Err(error) = client_message_rx
                .send(rocket_ws::Message::Binary(encoding_buffer.clone()))
                .await
            {
                tracing::error!("Failed to send ClientOutput message: {}", error);
                return GameRoomDisconnectReason::ClientDisconnected;
            }
        }
        GameRoomDisconnectReason::ConnectionClosedByHost
    }

    /***
     * Reads messages from the Client
     * Writes to the GameRoom
     */
    async fn client_reader_task(
        client_handle: ClientHandle,
        mut client_message_tx: SplitStream<DuplexStream>,
        client_to_game_sender: tokio::sync::mpsc::Sender<ClientInput>,
    ) -> GameRoomDisconnectReason {
        while let Some(message) = client_message_tx.next().await {
            if let Err(error) = &message {
                tracing::error!("{}", error);
                return GameRoomDisconnectReason::UnexpectedError(error.to_string());
            }

            match message.unwrap() {
                rocket_ws::Message::Binary(data) => {
                    let proto_msg = match prost_types::Any::decode(data.as_slice()) {
                        Err(error) => {
                            tracing::warn!("Message decoding failed: {}", error);
                            continue;
                        }
                        Ok(msg) => msg,
                    };

                    let proto_client_input = ClientInput {
                        client_id: Some(ClientId {
                            value: client_handle.0,
                        }),
                        game_input_message: Some(proto_msg),
                    };

                    if let Err(error) = client_to_game_sender.send(proto_client_input).await {
                        tracing::error!("Failed to send ClientInput message: {}", error);
                        continue;
                    }
                }
                rocket_ws::Message::Close(_) => {
                    return GameRoomDisconnectReason::ClientClosedConnection;
                }
                _ => {
                    tracing::warn!("Unexpected message type.");
                }
            }
        }
        GameRoomDisconnectReason::ClientDisconnected
    }
}
