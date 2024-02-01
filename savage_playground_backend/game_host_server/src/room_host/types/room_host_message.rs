use super::{client_handle::ClientHandle, message::Message, room_handle::RoomHandle};

#[derive(Clone, Debug)]
pub enum RoomHostMessage {
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
