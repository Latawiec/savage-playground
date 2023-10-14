use serde::{Serialize, Deserialize};

use crate::server::client::ClientID;


#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    StartGame { game_name: String },
    SetGameOwner { new_game_owner: ClientID },
    
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    GameOwnerChanged { game_owner: ClientID },
    GameError { reason: String },
    GameState {
        #[serde(flatten)]
        content: serde_json::Value
    }
}