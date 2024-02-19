use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use prost::Message;
use rocket_ws::stream::DuplexStream;
use room_server_interface::proto::{
    client_input::ClientInput, client_output::ClientOutput, common::ClientId,
};

use crate::server::game_host::types::ClientHandle;

#[derive(Clone, Debug)]
pub enum CloseReason {
    ClosedByClient,
    ClientConnectionDestroyed,
    ReadNone,
    HostClosedChannel,
    Error(String),
}

pub struct ConnectionCloseHandle {
    closed_receiver: tokio::sync::mpsc::Receiver<Option<CloseReason>>,
}

impl ConnectionCloseHandle {
    pub async fn wait(&mut self) -> Option<CloseReason> {
        self.closed_receiver.recv().await.flatten()
    }
}

pub struct ClientConnectionHandle {
    closed_sender: tokio::sync::mpsc::Sender<Option<CloseReason>>,
    input_task_handle: tokio::task::JoinHandle<()>,
    output_task_handle: tokio::task::JoinHandle<()>,
}

pub struct ClientConnection;

impl Drop for ClientConnectionHandle {
    fn drop(&mut self) {
        self.input_task_handle.abort();
        self.output_task_handle.abort();
        let _ = self
            .closed_sender
            .blocking_send(Some(CloseReason::ClientConnectionDestroyed));
    }
}

impl ClientConnection {
    pub fn start(
        client_handle: ClientHandle,
        ws_stream: DuplexStream,
        input_sender: tokio::sync::mpsc::Sender<ClientInput>,
        output_receiver: tokio::sync::mpsc::Receiver<ClientOutput>,
    ) -> (ClientConnectionHandle, ConnectionCloseHandle) {
        println!("HALO");
        let (closed_sender, closed_receiver) = tokio::sync::mpsc::channel::<Option<CloseReason>>(1);
        let (mut rx, mut tx) = ws_stream.split();

        let closed_sender_clone = closed_sender.clone();
        let input_task_handle = tokio::spawn(async move {
            println!("Helo?");
            let closed_sender = closed_sender_clone;
            while let Some(value_read) = tx.next().await {
                println!("Read: {:?}", value_read);
                match value_read {
                    Err(error) => {
                        let _ = closed_sender
                            .send(Some(CloseReason::Error(error.to_string())))
                            .await;
                        return;
                    }
                    Ok(message) => {
                        match message {
                            rocket_ws::Message::Binary(data) => {
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
                            }
                            rocket_ws::Message::Close(_) => {
                                let _ = closed_sender.send(Some(CloseReason::ClosedByClient)).await;
                                return;
                            }
                            _ => {
                                // Unexpected message format... let it slide for now.
                            }
                        }
                    }
                }
            }
            let _ = closed_sender.send(Some(CloseReason::ReadNone)).await;
        });

        let closed_sender_clone = closed_sender.clone();
        let output_task_handle = tokio::spawn(async move {
            let mut output_receiver = output_receiver;
            let closed_sender = closed_sender_clone;
            while let Some(msg) = output_receiver.recv().await {
                let proto_msg = msg.encode_to_vec();

                if let Err(error) = rx.send(rocket_ws::Message::Binary(proto_msg)).await {
                    let _ = closed_sender
                        .send(Some(CloseReason::Error(error.to_string())))
                        .await;
                    return;
                }
            }
            let _ = closed_sender
                .send(Some(CloseReason::HostClosedChannel))
                .await;
        });

        let connection = ClientConnectionHandle {
            closed_sender,
            input_task_handle,
            output_task_handle,
        };

        let close_handle = ConnectionCloseHandle { closed_receiver };

        (connection, close_handle)
    }
}
