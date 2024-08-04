use std::sync::Arc;

use crate::{game_host::types::ClientHandle, util::OnceNotify};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use game_interface::proto::{game_input::ClientInput, game_output::GameMessage};
use prost::Message;
use rocket_ws::stream::DuplexStream;
use tokio::task::JoinHandle;

use super::disconnect_reason::GameRoomDisconnectReason;

#[derive(Clone, Default)]
pub struct GameRoomConnectionHandle {
    client_handle: ClientHandle,
    close_notify: Arc<OnceNotify<GameRoomDisconnectReason>>,
    _actor: Arc<Option<GameRoomConnectionActor>>,
}

impl GameRoomConnectionHandle {
    pub fn new(
        client_handle: ClientHandle,
        client_connection: DuplexStream,
        client_message_tx: tokio::sync::mpsc::Sender<ClientInput>,
        client_message_rx: tokio::sync::mpsc::Receiver<GameMessage>,
        connection_span: tracing::Span,
    ) -> GameRoomConnectionHandle {
        let (rx, tx) = client_connection.split();
        let close_notify = Arc::new(OnceNotify::<GameRoomDisconnectReason>::default());

        let client_reader_task = tokio::spawn({
            let close_notify = close_notify.clone();
            let connection_span = connection_span.clone();
            async move {
                let close_reason = GameRoomConnectionActor::client_reader_task(
                    client_handle.clone(),
                    tx,
                    client_message_tx,
                    connection_span
                )
                .await;
                close_notify.notify(close_reason);
            }
        });

        let client_sender_task = tokio::spawn({
            let close_notify = close_notify.clone();
            let connection_span = connection_span.clone();
            async move {
                let close_reason = GameRoomConnectionActor::client_writer_task(
                    client_handle.clone(),
                    rx,
                    client_message_rx,
                    connection_span
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
            client_handle,
            close_notify,
            _actor,
        }
    }

    pub fn client_id(&self) -> u64 {
        self.client_handle.0
    }

    pub fn new_closed(close_reason: GameRoomDisconnectReason) -> GameRoomConnectionHandle {
        let close_notify = Arc::new(OnceNotify::<GameRoomDisconnectReason>::default());
        close_notify.notify(close_reason);
        GameRoomConnectionHandle {
            client_handle: ClientHandle(0),
            close_notify,
            _actor: Default::default(),
        }
    }

    pub async fn wait(&self) -> GameRoomDisconnectReason {
        self.close_notify.notified().await
    }

    pub fn close(&self, reason: GameRoomDisconnectReason) {
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
        mut game_to_client_receiver: tokio::sync::mpsc::Receiver<GameMessage>,
        connection_span: tracing::Span,
    ) -> GameRoomDisconnectReason {
        let mut encoding_buffer = Vec::<u8>::new();
        while let Some(msg) = game_to_client_receiver.recv().await {
            if let None = &msg.message {
                tracing::warn!(name: "client_writer_task", target: module_path!(), parent: &connection_span, "message is empty");
                continue;
            }

            if let Err(error) = msg.message.unwrap().encode(&mut encoding_buffer) {
                tracing::error!(name: "client_writer_task", target: module_path!(), parent: &connection_span, "message encoding error: {}", error);
                continue;
            }

            if let Err(error) = client_message_rx
                .send(rocket_ws::Message::Binary(encoding_buffer.clone()))
                .await
            {
                tracing::error!(name: "client_writer_task", target: module_path!(), parent: &connection_span, "failed to send message to client: {}", error);
                return GameRoomDisconnectReason::ClientDisconnected;
            }
            tracing::trace!(name: "client_writer_task", target: module_path!(), parent: &connection_span, bytes = encoding_buffer.len(), "message sent to client");
            encoding_buffer.clear();
        }
        tracing::info!(name: "client_writer_task", target: module_path!(), parent: &connection_span, "channel is closed");
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
        connection_span: tracing::Span,
    ) -> GameRoomDisconnectReason {
        while let Some(message) = client_message_tx.next().await {
            if let Err(error) = &message {
                tracing::error!(name: "client_reader_task", target: module_path!(), parent: &connection_span, "failed to receive client message: {}", error);
                return GameRoomDisconnectReason::UnexpectedError(error.to_string());
            }
            let message = message.unwrap();
            tracing::trace!(name: "client_reader_task", target: module_path!(), parent: &connection_span, bytes = message.len(), "message read from client");

            match message {
                rocket_ws::Message::Binary(data) => {
                    let proto_msg = match prost_types::Any::decode(data.as_slice()) {
                        Err(error) => {
                            tracing::error!(name: "client_reader_task", target: module_path!(), parent: &connection_span, "message decoding error: {}", error);
                            continue;
                        }
                        Ok(msg) => msg,
                    };

                    let proto_client_input = ClientInput {
                        sender_id: client_handle.0,
                        game_input_message: Some(proto_msg),
                    };

                    if let Err(error) = client_to_game_sender.send(proto_client_input).await {
                        tracing::error!(name: "client_reader_task", target: module_path!(), parent: &connection_span, "failed to send message to game room: {}", error);
                        continue;
                    }
                }
                rocket_ws::Message::Close(_) => {
                    return GameRoomDisconnectReason::ClientClosedConnection;
                }
                _ => {
                    tracing::error!(name: "client_reader_task", target: module_path!(), parent: &connection_span, "unexpected message type");
                }
            }
        }
        tracing::info!(name: "client_reader_task", target: module_path!(), parent: &connection_span, "channel is closed");
        GameRoomDisconnectReason::ClientDisconnected
    }
}
