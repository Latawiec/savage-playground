use super::{client_handle::ClientHandle, error::RoomHostError, room_handle::RoomHandle};

#[derive(Clone, Debug)]
pub enum RoomHostNotification {
    RoomCreated {
        room: RoomHandle,
    },
    RoomDestroyed {
        room: RoomHandle,
    },
    RoomEmpty {
        room: RoomHandle,
    },

    ClientCreated {
        client: ClientHandle,
    },
    ClientRemoved {
        client: ClientHandle,
    },
    ClientJoined {
        room: RoomHandle,
        client: ClientHandle,
    },
    ClientLeft {
        room: RoomHandle,
        client: ClientHandle,
    },
}

pub trait RoomHostInfo {
    async fn get_room_clients(&self, room: RoomHandle) -> Result<Vec<ClientHandle>, RoomHostError>;

    async fn get_rooms(&self) -> Result<Vec<RoomHandle>, RoomHostError>;

    fn subscribe(
        sender: tokio::sync::mpsc::Sender<RoomHostNotification>,
    ) -> Result<(), RoomHostError>;
}

pub trait RoomHostManagement {
    async fn create_client(&mut self) -> ClientHandle;

    async fn remove_client(&mut self) -> Result<(), RoomHostError>;

    async fn create_room(&mut self) -> RoomHandle;

    async fn update_room(&mut self, client: ClientHandle, room: RoomHandle) -> Result<(), RoomHostError>;

    async fn close_room(&mut self, client: ClientHandle, room: RoomHandle) -> Result<(), RoomHostError>;

    async fn join_room(&mut self, client: ClientHandle, room: RoomHandle) -> Result<(), RoomHostError>;

    async fn leave_room(&mut self, client: ClientHandle, room: RoomHandle) -> Result<(), RoomHostError>;
}
