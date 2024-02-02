#[derive(Debug)]
pub enum RoomHostError {
    Unknown,
    InternalError { message: String },
    StatePoisoned,

    ClientNotFound,
    ClientAlreadyInRoom,
    ClientNotInRoom,

    RoomNotFound,
    RoomAlreadyExists,

    RoomIsFull,
    InsufficientPermissions,
}
