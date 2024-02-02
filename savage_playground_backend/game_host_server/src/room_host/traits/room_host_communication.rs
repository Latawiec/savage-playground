use crate::room_host::types::{
    error::RoomHostError, room_client_message::RoomClientMessage,
    room_host_message::RoomHostMessage,
};

pub trait RoomHostInput {
    async fn send_client_msg(&self, client_message: RoomClientMessage)
        -> Result<(), RoomHostError>;

    fn subscribe_client_msg(
        &self,
    ) -> Result<tokio::sync::broadcast::Receiver<RoomClientMessage>, RoomHostError>;
}

pub trait RoomHostOutput {
    async fn send_host_msg(&self, host_message: RoomHostMessage)
        -> Result<(), RoomHostError>;

    fn subscribe_host_msg(
        &self,
    ) -> Result<tokio::sync::broadcast::Receiver<RoomHostMessage>, RoomHostError>;
}
