use super::client::ClientID;
use super::room::RoomID;

/// Message that comes from clients actions. Read and interpreted by the server.
#[derive(Clone, Debug)]
pub enum ClientMessage {
    Connected { client_id: ClientID },
    Disconnected{ client_id: ClientID },

    JoinedRoom { client_id: ClientID, room_id: RoomID },
    LeftRoom { client_id: ClientID, room_id: RoomID },
    
    String { client_id: ClientID, text: String },
    Binary { client_id: ClientID, data: Vec<u8> },
}

/// Message that comes from the server, sent over to the client.
#[derive(Clone, Debug)]
pub enum ServerMessage {
    /// Sent to every single client of the server.
    Broadcast {},
    /// Sent to clients being members of a specific server room.
    Room { room_id: RoomID },
    /// Sent to a specific single client.
    Client { client_id: ClientID },
}

/// Messages that are meant for internal use only.
pub (super) enum InternalMessage {
    
}