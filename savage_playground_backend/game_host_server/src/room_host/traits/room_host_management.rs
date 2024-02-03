use crate::room_host::types::{
    client_handle::ClientHandle, error::RoomHostError, room_handle::RoomHandle,
};

pub trait RoomHostManagement {
    fn create_client(
        &self,
    ) -> impl std::future::Future<Output = Result<ClientHandle, RoomHostError>> + Send;

    fn create_room(
        &self,
    ) -> impl std::future::Future<Output = Result<RoomHandle, RoomHostError>> + Send;

    fn remove_client(
        &self,
        client: ClientHandle,
    ) -> impl std::future::Future<Output = Result<(), RoomHostError>> + Send;

    fn remove_room(
        &self,
        room: RoomHandle,
    ) -> impl std::future::Future<Output = Result<(), RoomHostError>> + Send;

    fn join_room(
        &self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> impl std::future::Future<Output = Result<(), RoomHostError>> + Send;

    fn leave_room(
        &self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> impl std::future::Future<Output = Result<(), RoomHostError>> + Send;
}
