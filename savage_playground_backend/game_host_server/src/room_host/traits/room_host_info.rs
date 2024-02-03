use std::collections::BTreeSet;

use crate::room_host::types::{
    client_handle::ClientHandle, error::RoomHostError, room_handle::RoomHandle,
    room_host_notification::RoomHostNotification,
};

pub trait RoomHostInfo {
    fn get_room_clients(
        &self,
        room: RoomHandle,
    ) -> impl std::future::Future<Output = Result<BTreeSet<ClientHandle>, RoomHostError>> + Send;

    fn get_rooms(
        &self,
    ) -> impl std::future::Future<Output = Result<BTreeSet<RoomHandle>, RoomHostError>> + Send;

    fn get_client_rooms(
        &self,
        client: ClientHandle,
    ) -> impl std::future::Future<Output = Result<BTreeSet<RoomHandle>, RoomHostError>> + Send;

    fn get_clients(
        &self,
    ) -> impl std::future::Future<Output = Result<BTreeSet<ClientHandle>, RoomHostError>> + Send;

    fn subscribe_host_info(
        &self,
    ) -> Result<tokio::sync::broadcast::Receiver<RoomHostNotification>, RoomHostError>;
}
