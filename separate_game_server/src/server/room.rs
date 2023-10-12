use std::{collections::HashMap, sync::Arc, net::SocketAddr, time::Duration};

use futures_util::{StreamExt, SinkExt};
use tokio::sync::{broadcast, RwLock};
use warp::filters::ws::{WebSocket, Message};

use super::{message::{ServerMessage, ClientMessage}, client::{Client, ClientID}};


pub type RoomID = u64;

#[derive(Clone)]
pub struct RoomHandle {
    pub room_id: RoomID,
    pub (crate) clients: Arc<RwLock<HashMap<ClientID, Client>>>,

    pub (self) server_msg_sender: broadcast::Sender<ServerMessage>,
    pub (self) client_msg_sender: broadcast::Sender<ClientMessage>,
}

#[derive(Debug)]
pub enum RoomError {
    Unknown,
}

impl RoomHandle {
    const SERVER_MESSAGE_CHANNEL_CAPACITY: usize = 128;
    const CLIENT_MESSAGE_CHANNEL_CAPACITY: usize = 128;

    pub fn new(room_id: RoomID) -> RoomHandle {
        let (server_msg_sender, _) = broadcast::channel::<ServerMessage>(Self::SERVER_MESSAGE_CHANNEL_CAPACITY);
        let (client_msg_sender, _) = broadcast::channel::<ClientMessage>(Self::CLIENT_MESSAGE_CHANNEL_CAPACITY);

        RoomHandle { room_id, clients: Default::default(), server_msg_sender, client_msg_sender }
    }

    pub async fn is_empty(&self) -> bool {
        self.clients.read().await.is_empty()
    }

    pub fn receiver(&self) -> broadcast::Receiver<ClientMessage> {
        self.client_msg_sender.subscribe()
    }

    pub fn sender(&self) -> broadcast::Sender<ServerMessage> {
        self.server_msg_sender.clone()
    }

    pub (crate) async fn create_room_client(&self, client_id: ClientID, addr: Option<SocketAddr>, websocket: WebSocket) -> Result<(), RoomError> {
        let client_msg_sender = self.client_msg_sender.clone();
        let mut room_msg_receiver = self.server_msg_sender.subscribe();

        let client_id = client_id.clone();
        let room_id = self.room_id.clone();

        // Send ClientMessage that client is connected.
        if let Err(error) = client_msg_sender.send(ClientMessage::Connected{ client_id }) {
            tracing::error!("Couldn't send ClientMessage: {}", error);
        }

        // Send ClientMessage that client joined the room.
        if let Err(error) = client_msg_sender.send(ClientMessage::JoinedRoom { client_id, room_id }) {
            tracing::error!("Couldn't send ClientMessage: {}", error);
        }

        let (mut tx, mut rx) = websocket.split();
        let clients_map = self.clients.clone();

        // Task with a loop reading messages from the client.
        let message_read_task_handle = tokio::spawn(async move {
            while let Some(msg) = rx.next().await {
                match msg {
                    Ok(msg) => {
                        if msg.is_text() {
                            if let Err(error) = client_msg_sender.send(ClientMessage::Data { client_id, message: super::message::Message::Text { data: msg.to_str().unwrap().to_owned() } }) {
                                tracing::error!("Couldn't send ClientMessage: {}", error);
                            }
                        } else
                        if msg.is_binary() {
                            if let Err(error) = client_msg_sender.send(ClientMessage::Data { client_id, message: super::message::Message::Binary { data: msg.as_bytes().to_vec() } }) {
                                tracing::error!("Couldn't send ClientMessage: {}", error);
                            }
                        }
                    },
                    Err(error) => {
                        tracing::error!("Got error from client WebSocket: {}", error);
                    },
                }
            }

            // If we ever leave the loop, it means we've disconnected. And disconnected also means - left the room.
            if let Err(error) = client_msg_sender.send(ClientMessage::LeftRoom { client_id, room_id }) {
                tracing::error!("Couldn't send ClientMessage: {}", error);
            }

            if let Err(error) = client_msg_sender.send(ClientMessage::Disconnected { client_id }) {
                tracing::error!("Couldn't send ClientMessage: {}", error);
            }

            clients_map.write().await.remove(&client_id);
        });

        // Task with a loop writing messages to clients.
        let message_write_task_handle = tokio::spawn(async move {
            let self_room_id = room_id;
            let self_client_id = client_id;
            loop {
                match room_msg_receiver.recv().await {
                    Err(error) => {
                        tracing::error!("Got error from client WebSocket: {}", error);
                        break;
                    },
                    Ok(msg) => {
                        match msg {
                            ServerMessage::Broadcast { message } => {
                                tx.send(Message::text(format!("Broadcast: {:?}", message))).await;
                            },
                            ServerMessage::Room { room_id, message } => {
                                if self_room_id == room_id {
                                    tx.send(Message::text(format!("Room message: {:?}", message))).await;
                                }
                            },
                            ServerMessage::Client { client_id, message } => {
                                if self_client_id == client_id {
                                    tx.send(Message::text(format!("Direct message: {:?}", message))).await;
                                }
                            },
                        }
                    },
                }
            }
        });

        let client = Client {
            id: client_id,
            addr,
            message_read_task_handle,
            message_write_task_handle,
        };

        // Remember newly connected client.
        self.clients.write().await.insert(client_id, client);

        Ok(())
    }
}