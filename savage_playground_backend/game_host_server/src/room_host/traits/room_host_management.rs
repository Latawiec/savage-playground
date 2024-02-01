use crate::room_host::types::{
    client_handle::ClientHandle, error::RoomHostError, room_handle::RoomHandle,
};

pub trait RoomHostManagement {
    async fn create_client(&mut self) -> ClientHandle;

    async fn create_room(&mut self) -> RoomHandle;

    async fn remove_client(&mut self, client: ClientHandle) -> Result<(), RoomHostError>;

    async fn update_room(
        &mut self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError>;

    async fn remove_room(
        &mut self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError>;

    async fn join_room(
        &mut self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError>;

    async fn leave_room(
        &mut self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError>;
}
