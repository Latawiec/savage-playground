pub enum RoomHostError {
    Unknown,

    ClientNotFound,

    RoomNotFound,
    RoomAlreadyExists,

    RoomIsFull,
    InsufficientPermissions,
}
