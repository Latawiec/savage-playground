
#[derive(Clone, Debug)]
pub enum GameRoomDisconnectReason {
    Ok,
    ConnectionClosedByHost,
    ClientDisconnected,
    ClientClosedConnection,
    ClientConnectionDestroyed,
    GameCrashed,
    RoomClosed,
    RoomDoesNotExist,
    UnexpectedError(String),
}
