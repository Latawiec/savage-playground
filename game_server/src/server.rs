use std::{sync::Arc};

use futures_util::{FutureExt, SinkExt, StreamExt, TryFutureExt};
use tokio::{runtime, sync::broadcast, sync::mpsc, task::JoinHandle};
use warp::{
    filters::ws::{Message, WebSocket},
    Filter,
};

#[derive(Clone, Debug)]
pub enum ServerMessage {
    Broadcast,
    Direct,
}

#[derive(Clone, Debug)]
pub enum ClientMessage {
    Connected,
    Disconnected,
    Message,
}

#[derive(Clone)]
pub struct ServerHandle {
    pub server_msg_sender: broadcast::Sender<ServerMessage>,
    client_msg_sender: broadcast::Sender<ClientMessage>,
}

impl ServerHandle {
    const SOCKET_PATH: &str = "socket";
    const SERVER_MESSAGE_CHANNEL_CAPACITY: usize = 64;
    const CLIENT_MESSAGE_CHANNEL_CAPACITY: usize = 64;

    pub fn new(rt: &runtime::Handle) -> (ServerHandle, JoinHandle<()>) {
        let (server_msg_sender, _) =
            broadcast::channel::<ServerMessage>(Self::SERVER_MESSAGE_CHANNEL_CAPACITY);
        let (client_msg_sender, _) =
            broadcast::channel::<ClientMessage>(Self::CLIENT_MESSAGE_CHANNEL_CAPACITY);

        let server_msg_sender_clone = server_msg_sender.clone();
        let client_msg_sender_clone = client_msg_sender.clone();

        let socket = warp::path(Self::SOCKET_PATH)
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let client_message_sender = client_msg_sender_clone.clone();
                let server_message_receiver = server_msg_sender_clone.subscribe();
                ws.on_upgrade(move |ws| {
                    Self::start_websocket(ws, client_message_sender, server_message_receiver)
                })
            });

        let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

        let server_join_handle = rt.spawn(async move {
            warp::serve(socket.or(hello)).run(([127, 0, 0, 1], 3030)).await
        });

        (ServerHandle { server_msg_sender, client_msg_sender }, server_join_handle)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ClientMessage> {
        self.client_msg_sender.subscribe()
    }

    async fn start_websocket(
        websocket: WebSocket,
        client_message_sender: broadcast::Sender<ClientMessage>,
        mut server_message_receiver: broadcast::Receiver<ServerMessage>,
    ) {
        let _ = client_message_sender.send(ClientMessage::Connected);

        let (mut tx, mut rx) = websocket.split();
        let message_read_handle = tokio::spawn(async move {
            while let Some(msg) = rx.next().await {
                let _ = client_message_sender.send(ClientMessage::Message);
            }
            let _ = client_message_sender.send(ClientMessage::Disconnected);
        });

        let message_write_handle = tokio::spawn(async move {
            while let Ok(msg) = server_message_receiver.recv().await {
                if let Err(e) = tx.send(Message::text("s")).await {
                    // Websocket dead on error?
                    return;
                }
            }
        });

        let _ = tokio::join!(message_read_handle, message_write_handle);
    }
}
