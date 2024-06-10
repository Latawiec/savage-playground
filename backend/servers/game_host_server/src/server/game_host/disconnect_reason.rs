
#[derive(Clone, Debug)]
pub enum DisconnectReason {
    ClientDisconnected,
    ClientClosedConnection,
    ClientConnectionDestroyed,
    GameCrashed,
    RoomClosed,
    RoomDoesNotExist,
    UnexpectedError(String),
}
