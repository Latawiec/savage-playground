use super::{client_handle::ClientHandle, message::Message};

#[derive(Clone, Debug)]
pub struct RoomClientMessage {
    client: ClientHandle,
    message: Message,
}
