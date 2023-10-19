use serde::{Serialize, Deserialize};

use crate::server::client::ClientID;


#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    StartGame { game_name: String },
    SetGameOwner { new_game_owner: ClientID },
    GameConfig { config: serde_json::Value },
    GameInput { input: serde_json::Value },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    GameOwnerChanged { game_owner: ClientID },
    GameStarted,
    GameResumed,
    GamePaused,
    GameStopped,
    GameState {
        #[serde(flatten)]
        content: serde_json::Value
    },
    GameError { reason: String },
}