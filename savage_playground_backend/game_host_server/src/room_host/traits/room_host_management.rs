use crate::room_host::types::{
    client_handle::ClientHandle, error::RoomHostError, room_handle::RoomHandle,
};

pub trait RoomHostManagement {
    async fn create_client(&self) -> Result<ClientHandle, RoomHostError>;

    async fn create_room(&self) -> Result<RoomHandle, RoomHostError>;

    async fn remove_client(&self, client: ClientHandle) -> Result<(), RoomHostError>;

    async fn remove_room(
        &self,
        room: RoomHandle,
    ) -> Result<(), RoomHostError>;

    async fn join_room(
        &self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError>;

    async fn leave_room(
        &self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError>;
}
