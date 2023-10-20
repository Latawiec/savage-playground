use std::path::{PathBuf, Path};

use tokio::{process::{ChildStdin, ChildStdout, ChildStderr}, io::AsyncWriteExt};

use crate::{instance::instance::Instance, server::{client::ClientID, room::RoomHandle, message::Message}};

use super::message::{self, Request};

mod error {

    #[derive(Debug)]
    pub enum GamesMappingError {
        GameMappingFileError { reason: String },
        GameDirError { reason: String },
        FileContentIllFormed { resason: String },
        GameNotFound { reason: String },
    }
    
    #[derive(Debug)]
    pub enum GameAuthorithyError {
        NotAuthorized { reason: String },
    }

    #[derive(Debug)]
    pub enum GameInstanceError {
        Unknown { reason: String }
    }

    #[derive(Debug)]
    pub enum GameError {
        NoGameRunning,
        StdInClosed,
        StdOutClosed,
        StdErrClosed,
    }
}

pub struct GameHost {
    game_owner: ClientID,
    room_handle: RoomHandle,

    game_dir: Option<PathBuf>,
    game_instance: Option<Instance>,

    game_stdin: Option<ChildStdin>,
    game_stdout: Option<ChildStdout>,
    game_stderr: Option<ChildStderr>,
}

impl GameHost {
    pub fn new(owner_id: ClientID, room_handle: RoomHandle) -> GameHost {
        GameHost {
            game_owner: owner_id,
            room_handle,
            game_dir: None,
            game_instance: None,
            game_stdin: None,
            game_stdout: None,
            game_stderr: None,
        }
    }

    pub async fn serve(&mut self) {

        let mut receiver = self.room_handle.receiver();
        let _sender = self.room_handle.sender();

        while let Ok(msg) = receiver.recv().await {
            match msg {
                crate::server::message::ClientMessage::Data { client_id, message } => {
                    if let Some(request) = Self::parse_client_message(message) {
                        match request {
                            Request::StartGame { game_name } => {
                                if self.game_owner != client_id {
                                    // let _ = Err(error::GameAuthorithyError::NotAuthorized{ reason: "Only current game owner can start the game.".to_owned() });
                                    // Pass error somehow.
                                    return;
                                }

                                let game_path = Self::try_get_game_dir(&game_name).await;
                                if let Err(error) = game_path {
                                    // Err(error)
                                    return;
                                }

                                let game_path = game_path.unwrap();
                                let game_instance = Self::try_start_game(&game_path).await;
                                if let Err(error) = game_instance {
                                    // Err(error)
                                    return;
                                }

                                let mut game_instance = game_instance.unwrap();
                                self.game_stdin = game_instance.take_stdin();
                                self.game_stdout = game_instance.take_stdout();
                                self.game_stderr = game_instance.take_stderr();
                                self.game_instance = Some(game_instance);
                                self.game_dir = Some(game_path);
                            },
                            Request::StopGame => {
                                if self.game_owner != client_id {
                                    // let _ = Err(error::GameAuthorithyError::NotAuthorized{ reason: "Only current game owner can start the game.".to_owned() });
                                    // Pass error somehow.
                                    return;
                                }

                                if self.game_instance.is_none() {
                                    // Err(error::GameError::NoGameRunning)
                                    return;
                                }

                                self.game_dir = None;
                                self.game_stdin = None;
                                self.game_stdout = None;
                                self.game_stderr = None;
                                self.game_instance = None;

                            },
                            Request::SetGameOwner { new_game_owner } => {
                                if self.game_owner != client_id {
                                    // let _ = Err(error::GameAuthorithyError::NotAuthorized{ reason: "Only current game owner can assign a new owner.".to_owned() });
                                    // Pass error somehow.
                                    return;
                                }
                        
                                self.set_game_owner(client_id, new_game_owner).await;
                            },
                            Request::GameMessage { message } => {
                                if self.game_instance.is_none() {
                                    // Err(error::GameError::NoGameRunning);
                                    // Pass error somehow
                                    return;
                                }
                                
                                let stdin = self.game_stdin.as_mut();
                                if stdin.is_none() {
                                    // Err(error::GameError::StdInClosed);
                                    return;
                                }

                                let stdin = stdin.unwrap();

                                let serialized_message = message.to_string();
                                let _ = stdin.write_all(serialized_message.as_bytes()).await;
                            }
                        };
                    } else {
                        tracing::warn!("Unsupported message format");
                    }
                },
                _ => {},
            };
        }
        
    }

    fn parse_client_message(message: Message) -> Option<message::Request> {
        match message {
            Message::Binary { data: _ } => None, // I dont use binary format yet
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

    async fn set_game_owner(&mut self, _client_id: u64, new_owner_id: u64) {
        self.game_owner = new_owner_id;
    }

    async fn try_start_game(game_dir: &Path) -> Result<Instance, error::GameInstanceError> {
        match Instance::new(&game_dir) {
            Ok(instance) => Ok(instance),
            Err(error) => {
                match error {
                    crate::instance::instance::Error::StartupError { reason } => Err(error::GameInstanceError::Unknown { reason }),
                    crate::instance::instance::Error::ProcessError { reason } => Err(error::GameInstanceError::Unknown { reason }),
                }
            }
        }
    }

    async fn try_get_game_dir(game_name: &str) -> Result<PathBuf, error::GamesMappingError> {
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
                if let Some(game_dir) = game_dir_map.get(game_name) {
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
