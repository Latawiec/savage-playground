use std::{sync::Arc, sync::OnceLock};

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

#[derive(Clone, Default)]
pub struct GameRoomConnectionHandle {
    close_notify: Arc<tokio::sync::Notify>,
    close_reason: Arc<OnceLock<GameRoomDisconnectReason>>,
}

impl GameRoomConnectionHandle {
    pub async fn close_fut(&self) -> GameRoomDisconnectReason {
        self.close_notify.notified().await;
        self.close_reason.get().expect("Close reason empty after being notified.").clone()
    }
}

pub struct GameRoomConnection {
    // Runtime.
    client_reader_task: JoinHandle<()>,
    client_sender_task: JoinHandle<()>,

    // Closing.
    connection_handle: GameRoomConnectionHandle,
}

/***
 * Handles reading and writing of data sent by the connected client.
 */
impl GameRoomConnection {
    pub fn new(
        client_handle: ClientHandle,
        client_connection: DuplexStream,
        client_message_tx: tokio::sync::mpsc::Sender<ClientInput>,
        client_message_rx: tokio::sync::mpsc::Receiver<ClientOutput>,
    ) -> GameRoomConnection {
        let (rx, tx) = client_connection.split();
        let close_notify: Arc<tokio::sync::Notify> = Default::default();
        let close_reason: Arc<OnceLock<GameRoomDisconnectReason>> = Default::default();
        let connection_handle = GameRoomConnectionHandle::default();

        let client_reader_task = {
            let connection_handle = connection_handle.clone();
            tokio::spawn(async move {
                let _ = connection_handle.close_reason.set(
                    Self::client_reader_task(client_handle.clone(), tx, client_message_tx).await,
                );
                connection_handle.close_notify.notify_waiters();
            })
        };

        let client_sender_task = {
            let connection_handle = connection_handle.clone();
            tokio::spawn(async move {
                let _ = connection_handle.close_reason.set(
                    Self::client_writer_task(client_handle.clone(), rx, client_message_rx).await,
                );
                connection_handle.close_notify.notify_waiters();
            })
        };

        GameRoomConnection {
            client_reader_task,
            client_sender_task,
            connection_handle
        }
    }

    pub fn close_connection(self) {
        let _ = self
            .close_reason
            .set(GameRoomDisconnectReason::ConnectionClosedByHost);
        self.close_notify.notify_waiters();
        self.client_reader_task.abort();
        self.client_sender_task.abort();
    }

    pub fn get_connection_handle(&self) {
        self.connection_handle.clone()
    }

    pub async fn close_fut(self) -> GameRoomDisconnectReason {
        self.connection_handle.close_fut().await
    }

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

impl Drop for GameRoomConnection {
    fn drop(&mut self) {
        let _ = self
            .close_reason
            .set(GameRoomDisconnectReason::ClientConnectionDestroyed);
        self.close_notify.notify_waiters();
        self.client_reader_task.abort();
        self.client_sender_task.abort();
    }
}
