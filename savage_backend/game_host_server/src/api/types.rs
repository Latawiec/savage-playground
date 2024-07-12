use rocket::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct GameRoomConfig {
    pub game_id: String,
    pub passcode: Option<u16>,
}

#[derive(Serialize)]
pub struct RoomsData {
    pub helo: bool
}