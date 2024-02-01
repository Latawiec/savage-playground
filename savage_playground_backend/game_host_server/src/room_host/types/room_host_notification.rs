use super::{client_handle::ClientHandle, room_handle::RoomHandle};

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
