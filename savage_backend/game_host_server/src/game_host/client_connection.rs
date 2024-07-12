use std::sync::{atomic::AtomicBool, Arc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use prost::Message;
use rocket_ws::stream::DuplexStream;
use room_server_interface::proto::{
    client_input::ClientInput, client_output::ClientOutput, common::ClientId,
};
use tokio::sync::OnceCell;

use crate::server::game_host::types::ClientHandle;

use super::game_room::disconnect_reason::GameRoomDisconnectReason;

#[derive(Clone)]
pub struct ClientConnectionHandle {
    closed: Arc<AtomicBool>,
    input_task_handle: Arc<tokio::task::JoinHandle<()>>,
    output_task_handle: Arc<tokio::task::JoinHandle<()>>,
    pub close_notify: Arc<tokio::sync::Notify>,
    pub close_reason: Arc<OnceCell<GameRoomDisconnectReason>>,
}

impl ClientConnectionHandle {
    pub fn new(
        client_handle: ClientHandle,
        ws_stream: DuplexStream,
        input_sender: tokio::sync::mpsc::Sender<ClientInput>,
        output_receiver: tokio::sync::mpsc::Receiver<ClientOutput>,
    ) -> ClientConnectionHandle {
        let (rx, tx) = ws_stream.split();
        let close_notify = Arc::new(tokio::sync::Notify::new());
        let close_reason = Arc::new(OnceCell::default());

        let input_task_handle = tokio::spawn(Self::input_task(
            client_handle,
            tx,
            input_sender,
            close_notify.clone(),
            close_reason.clone(),
        ));
        let output_task_handle = tokio::spawn(Self::output_task(
            client_handle,
            rx,
            output_receiver,
            close_notify.clone(),
            close_reason.clone(),
        ));

        ClientConnectionHandle {
            closed: Default::default(),
            input_task_handle: Arc::new(input_task_handle),
            output_task_handle: Arc::new(output_task_handle),
            close_notify,
            close_reason,
        }
    }

    pub fn disconnect(&self, reason: GameRoomDisconnectReason) {
        if self.closed.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.input_task_handle.abort();
        self.output_task_handle.abort();
        if let Ok(_) = self.close_reason.set(reason) {
            self.close_notify.notify_waiters();
        }
    }

    async fn input_task(
        client_handle: ClientHandle,
        mut tx: SplitStream<DuplexStream>,
        input_sender: tokio::sync::mpsc::Sender<ClientInput>,
        close_notify: Arc<tokio::sync::Notify>,
        close_reason: Arc<OnceCell<GameRoomDisconnectReason>>,
    ) {
        while let Some(value_read) = tx.next().await {
            println!("Read: {:?}", value_read);
            match value_read {
                Err(error) => {
                    println!("Err");
                    let _ = close_reason.set(GameRoomDisconnectReason::UnexpectedError(error.to_string()));
                    break;
                }
                Ok(message) => {
                    match message {
                        rocket_ws::Message::Binary(data) => {
                            println!("Bin");
                            // Expect proto.
                            let proto_msg = match prost_types::Any::decode(data.as_slice()) {
                                Ok(msg) => msg,
                                Err(error) => {
                                    tracing::error!("Message decoding failed: {}", error);
                                    continue;
                                }
                            };

                            let proto_client_input = ClientInput {
                                client_id: Some(ClientId {
                                    value: client_handle.0,
                                }),
                                game_input_message: Some(proto_msg),
                            };
                            let _ = input_sender.send(proto_client_input).await;
                            println!("Sent to pipe.");
                        }
                        rocket_ws::Message::Close(_) => {
                            println!("close");
                            let _ = close_reason.set(GameRoomDisconnectReason::ClientClosedConnection);
                            break;
                        }
                        _ => {
                            println!("Unexpected.");
                            // Unexpected message format... let it slide for now.
                        }
                    }
                }
            }
        }
        let _ = close_reason.set(GameRoomDisconnectReason::ClientDisconnected);
        close_notify.notify_waiters();
    }

    async fn output_task(
        _client_handle: ClientHandle,
        mut rx: SplitSink<DuplexStream, rocket_ws::Message>,
        output_receiver: tokio::sync::mpsc::Receiver<ClientOutput>,
        close_notify: Arc<tokio::sync::Notify>,
        close_reason: Arc<OnceCell<GameRoomDisconnectReason>>,
    ) {
        let mut output_receiver = output_receiver;
        while let Some(msg) = output_receiver.recv().await { 
            println!("Read from pipe.");
            let proto_msg = msg.game_output_message.unwrap().encode_to_vec();

            if let Err(error) = rx.send(rocket_ws::Message::Binary(proto_msg)).await {
                let _ = close_reason.set(GameRoomDisconnectReason::UnexpectedError(error.to_string()));
                break;
            }
        }
        let _ = close_reason.set(GameRoomDisconnectReason::ClientDisconnected);
        close_notify.notify_waiters();
    }
}

impl Drop for ClientConnectionHandle {
    fn drop(&mut self) {
        self.disconnect(GameRoomDisconnectReason::ClientConnectionDestroyed);
    }
}