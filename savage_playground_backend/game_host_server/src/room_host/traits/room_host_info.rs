use std::collections::BTreeSet;

use crate::room_host::types::{
    client_handle::ClientHandle, error::RoomHostError, room_handle::RoomHandle,
    room_host_notification::RoomHostNotification,
};

pub trait RoomHostInfo {
    async fn get_room_clients(&self, room: RoomHandle)
        -> Result<BTreeSet<ClientHandle>, RoomHostError>;

    async fn get_rooms(&self)
        -> Result<BTreeSet<RoomHandle>, RoomHostError>;

    async fn get_client_rooms(&self, client: ClientHandle)
        -> Result<BTreeSet<RoomHandle>, RoomHostError>;

    async fn get_clients(&self)
        -> Result<BTreeSet<ClientHandle>, RoomHostError>;
        
    fn subscribe_host_info(&self)
        -> Result<tokio::sync::broadcast::Receiver<RoomHostNotification>, RoomHostError>;
}
