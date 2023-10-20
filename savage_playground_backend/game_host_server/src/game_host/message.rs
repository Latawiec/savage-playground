use serde::{Serialize, Deserialize};

use crate::server::client::ClientID;


#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    StartGame { game_name: String },
    StopGame,
    SetGameOwner { new_game_owner: ClientID },
    // Message for game, we can't understand it at this level. Forward
    GameMessage {
        // We get a string, then deserialize it to serde_json::Value and then serialize back to string
        // to send it over. I'd much rather skip this step but without it game would have to check
        // validity of the message and its formatting. I'll leave it for now.
        message: serde_json::Value
    },
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