use super::{client_handle::ClientHandle, message::Message};

#[derive(Clone, Debug)]
pub struct RoomClientMessage {
    pub client: ClientHandle,
    pub message: Message,
}
