use std::fmt::Display;


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

impl Display for GameRoomDisconnectReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameRoomDisconnectReason::Ok => f.write_str(""),
            GameRoomDisconnectReason::ConnectionClosedByHost => f.write_str("connection closed by host"),
            GameRoomDisconnectReason::ClientDisconnected => f.write_str("client disconnected"),
            GameRoomDisconnectReason::ClientClosedConnection => f.write_str("client closed connection"),
            GameRoomDisconnectReason::ClientConnectionDestroyed => f.write_str("client connection destroyed"),
            GameRoomDisconnectReason::GameCrashed => f.write_str("game crashed"),
            GameRoomDisconnectReason::RoomClosed => f.write_str("room closed"),
            GameRoomDisconnectReason::RoomDoesNotExist => f.write_str("room does not exist"),
            GameRoomDisconnectReason::UnexpectedError(err) => f.write_fmt(format_args!("unexpected error: {}", err)),
        }
    }
}