use super::{client_handle::ClientHandle, error::RoomHostError, room_handle::RoomHandle};

#[derive(Clone, Debug)]
pub enum Message {
    Text { data: String },
    Binary { data: Vec<u8> },
}

pub enum HostMessage {
    Broadcast {
        message: Message,
    },
    RoomMessage {
        room: RoomHandle,
        message: Message,
    },
    ClientMessage {
        client: ClientHandle,
        message: Message,
    },
}

// Carries messages from the Room Host to it's Clients.
pub trait RoomHostMessenger {
    async fn send(host_message: HostMessage) -> Result<(), RoomHostError>;
}

#[derive(Clone, Debug)]
pub struct ClientMessage {
    client: ClientHandle,
    message: Message,
}

// Carries messages from Clients to the Room Host.
pub trait RoomClientMessenger {
    async fn send(client_message: ClientMessage) -> Result<(), RoomHostError>;
    fn subscribe(sender: tokio::sync::mpsc::Sender<ClientMessage>) -> Result<(), RoomHostError>;
}
