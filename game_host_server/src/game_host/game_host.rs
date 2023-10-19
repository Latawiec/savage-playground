use std::path::{PathBuf, Path};

use crate::{instance::instance::Instance, server::{client::ClientID, room::RoomHandle, message::Message}};

use super::message::{self, Response, Request};

mod error {

    pub enum GamesMappingError {
        GameMappingFileError { reason: String },
        GameDirError { reason: String },
        FileContentIllFormed { resason: String },
        GameNotFound { reason: String },
    }
    
    pub enum GameAuthorithyError {
        NotAuthorized { reason: String },
    }

    pub enum GameInstanceError {
        Unknown
    }
}

pub struct GameHost {
    game_owner: ClientID,
    room_handle: RoomHandle,

    game_dir: Option<PathBuf>,
    game_instance: Option<Instance>,
}

impl GameHost {
    pub fn new(owner_id: ClientID, room_handle: RoomHandle) -> GameHost {
        GameHost {
            game_owner: owner_id,
            room_handle,
            game_dir: None,
            game_instance: None,
        }
    }

    pub async fn serve(&mut self) {

        let mut receiver = self.room_handle.receiver();
        let mut sender = self.room_handle.sender();

        while let Ok(msg) = receiver.recv().await {
            match msg {
                crate::server::message::ClientMessage::Data { client_id, message } => {
                    if let Some(request) = Self::parse_client_message(message) {
                        match request {
                            Request::StartGame { game_name } => {
                                if self.owner_id != client_id {
                                    let error = Err(error::GameAuthorithyError{ reason: "Only current game owner can start the game.".to_owned() });
                                    // Pass error somehow.
                                }
                        
                                let _ = self.start_game(game_name).await;
                            },
                            Request::SetGameOwner { new_game_owner } => {
                                if self.owner_id != client_id {
                                    let error = Err(error::GameAuthorithyError{ reason: "Only current game owner can assign a new owner.".to_owned() });
                                    // Pass error somehow.
                                }
                        
                                self.set_game_owner(client_id, new_game_owner).await;
                            },
                            Request::GameConfig { config } => todo!(),
                            Request::GameInput { input } => todo!(),
                        };
                    } else {

                    }
                },
                _ => {},
            };
        }
        
    }

    fn parse_client_message(message: Message) -> Option<message::Request> {
        match message {
            Message::Binary { data } => None, // I dont use binary format yet
            Message::Text { data } => {
                match serde_json::from_str::<Request>(&data) {
                    Ok(request) => Some(request),
                    Err(err) => {
                        tracing::error!("Couldn't deserialize message: {:?}", err);
                        None
                    },
                }
            },
        }
    }

    async fn set_game_owner(&mut self, client_id: u64, new_owner_id: u64) {
        self.game_owner = new_owner_id;
    }

    async fn start_game(&mut self, game_name: String) -> Result<Instance, error::GameInstanceError> {
        Err(error::GameInstanceError::Unknown)
    }

    async fn try_get_game_dir(game_name: String) -> Result<PathBuf, error::GamesMappingError> {
        const GAME_DIR_MAPPING_FILENAME: &str = "game_dir_mapping.json";

        let games_mapping_json = match tokio::fs::read_to_string(GAME_DIR_MAPPING_FILENAME).await {
            Ok(game_dir_mapping) => {
                match serde_json::from_str::<serde_json::Value>(&game_dir_mapping) {
                    Ok(mapping) => mapping,
                    Err(error) => {
                        return Err(error::GamesMappingError::FileContentIllFormed { resason: error.to_string() });
                    },
                }
            },
            Err(error) => {
                return Err(error::GamesMappingError::GameMappingFileError { reason: error.to_string() });
            },
        };

        let game_dir = match games_mapping_json.as_object() {
            Some(game_dir_map) => {
                if let Some(game_dir) = game_dir_map.get(&game_name) {
                    game_dir
                } else {
                    return Err(error::GamesMappingError::GameNotFound { reason: format!("Game {} doesn't exist in the mapping.", game_name) });
                }
            },
            None => {
                return Err(error::GamesMappingError::FileContentIllFormed { resason: "Expected object as root of the mapping".to_owned() });
            },
        };

        let game_dir = match game_dir {
            serde_json::Value::String(game_dir) => game_dir,
            _ => {
                return Err(error::GamesMappingError::FileContentIllFormed { resason: "Expected String as game mapping".to_owned() });
            },
        };

        let game_dir_path = Path::new(game_dir);

        if !tokio::fs::try_exists(game_dir_path).await.unwrap() {
            return Err(error::GamesMappingError::GameDirError { reason: "Directory does not exist".to_owned() });
        }

        // Can't find tokio replacement for this one.
        if !game_dir_path.is_dir() {
            return Err(error::GamesMappingError::GameDirError { reason: "Path doesn't point to a directory".to_owned() });
        }
        
        Ok(game_dir_path.to_owned())
    }
}
