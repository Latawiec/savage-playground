use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use room_server_interface::schema::game_config::GameConfig;
use serde::{Deserialize, Serialize};

use crate::room_server::{
    client::ClientID,
    room::{RoomHandle, RoomID},
    room_server_handler::RoomServerBackend,
    server::{self, RoomServerHandle},
};

use super::game_host::GameHost;

mod error {

    #[derive(Debug)]
    pub enum GameDirMappingError {
        FileError { reason: String },
        UnexpectedFormat { reason: String },
        ConfigError { reason: String },
        GameNotFound { reason: String },
    }

    use warp::{http::StatusCode, reject::Reject, reply::Reply, Rejection};

    #[derive(Debug)]
    pub enum Error {
        GameConfigIllFormed,
        GameNotFound,
    }

    impl Reject for Error {}
}

#[derive(Serialize, Deserialize, Clone)]
struct GameDirConfig {
    cwd: PathBuf,
    runnable: PathBuf,
}

#[derive(Serialize, Deserialize, Default)]
struct GameDirMapping(HashMap<String, GameDirConfig>);

#[derive(Clone)]
pub struct GameHostManagerHandle {
    pub(self) game_hosts: Arc<RwLock<BTreeMap<RoomID, GameHost>>>,
    pub(self) game_configs: Arc<RwLock<BTreeMap<RoomID, GameConfig>>>,
    pub(self) game_dir_mapping: Arc<RwLock<GameDirMapping>>,
    pub(self) game_dir_mapping_file: PathBuf,
}

impl RoomServerBackend for GameHostManagerHandle {
    fn create_room(
        &self,
        client_id: ClientID,
        room_handle: RoomHandle,
    ) -> Result<(), warp::Rejection> {
        let mut room_handles_lock = self
            .room_handles
            .write()
            .map_err(|e| warp::reject::custom(server::error::Error::InternalError))?;

        if room_handles_lock.contains_key(&room_id) {
            return Err(warp::reject::custom(
                server::error::Error::RoomAlreadyExists,
            ));
        }

        let mut game_hosts_lock = self
            .game_hosts
            .write()
            .map_err(|e| warp::reject::custom(server::error::Error::InternalError))?;

        let room_handle = RoomHandle::new(room_id);
        let game_host = GameHost::new(client_id, room_handle.clone());

        room_handles_lock.insert(room_id, room_handle.clone());
        game_hosts_lock.insert(room_id, game_host);

        Ok(room_handle)
    }

    fn update_room(
        &self,
        room_id: RoomID,
        client_id: ClientID,
        config: serde_json::Value,
    ) -> Result<(), warp::Rejection> {
        let game_config = serde_json::from_value::<
            room_server_interface::schema::game_config::GameConfig,
        >(config)
        .map_err(|e| warp::reject::custom(error::Error::GameConfigIllFormed))?;

        // Empty, we assume they want to kill the game. Because my GameConfig isn't amazingly defined yet.
        match &game_config.game_path {
            Some(game_path) => {
                let game_dir_mapping_lock = self
                    .game_dir_mapping
                    .read()
                    .map_err(|e| warp::reject::custom(server::error::Error::InternalError))?;

                if !game_dir_mapping_lock.0.contains_key(game_path) {
                    return Err(warp::reject::custom(error::Error::GameNotFound));
                }

                let game_dir_config = game_dir_mapping_lock.0.get(game_path).unwrap();

                let game_cwd = &game_dir_config.cwd;
                let game_runnable = &game_dir_config.runnable;

                let mut game_hosts_lock = self
                    .game_hosts
                    .write()
                    .map_err(|e| warp::reject::custom(server::error::Error::InternalError))?;

                if let Some(game_host) = game_hosts_lock.get_mut(&room_id) {
                    if let Err(error) = game_host.start(&game_cwd, game_runnable, &vec![]) {
                        tracing::error!("Couldn't start the game {game_path}: {:?}", error);
                        game_host.stop();
                        return Err(warp::reject::custom(error::Error::GameNotFound));
                    }
                }
            }
            None => {
                let mut game_hosts_lock = self
                    .game_hosts
                    .write()
                    .map_err(|e| warp::reject::custom(server::error::Error::InternalError))?;

                if let Some(game_host) = game_hosts_lock.get_mut(&room_id) {
                    game_host.stop();
                }
            }
        }

        // Save this config.
        let mut game_configs_lock = self
            .game_configs
            .write()
            .map_err(|e| warp::reject::custom(server::error::Error::InternalError))?;
        game_configs_lock.insert(room_id, game_config);

        Ok(())
    }

    fn join_room(&self, room_id: RoomID, client_id: ClientID) -> Result<(), warp::Rejection> {
        let room_handles_lock = self
            .room_handles
            .read()
            .map_err(|e| warp::reject::custom(server::error::Error::InternalError))?;

        if !room_handles_lock.contains_key(&room_id) {
            return Err(warp::reject::custom(
                server::error::Error::RoomDoesNotExist,
            ));
        }

        let room_handle = room_handles_lock.get(&room_id).unwrap();
        room_handle.create_room_client(client_id, addr, websocket)

        Ok(())
    }

    fn leave_room(&self, room_id: RoomID, client_id: ClientID) -> Result<(), warp::Rejection> {
        todo!()
    }
}

impl GameHostManagerHandle {
    pub fn new(
        server_handle: RoomServerHandle,
        game_dir_mapping_file: &Path,
    ) -> Option<GameHostManagerHandle> {
        let game_host_manager_handle = GameHostManagerHandle {
            room_handles: Default::default(),
            game_hosts: Default::default(),
            game_configs: Default::default(),
            game_dir_mapping: Default::default(),
            game_dir_mapping_file: game_dir_mapping_file.to_owned(),
        };

        Some(game_host_manager_handle)
    }

    pub async fn reload_game_dir_mapping(&mut self) {
        self.load_game_dir_mapping();
    }

    async fn load_game_dir_mapping(&mut self) -> Result<(), error::GameDirMappingError> {
        const GAME_HOST_SERVER_GAMES_DIR: &str = "GAME_HOST_SERVER_GAMES_DIR";
        let game_host_server_games_dir = match std::env::var(GAME_HOST_SERVER_GAMES_DIR) {
            Ok(path) => PathBuf::from(path),
            Err(_) => PathBuf::new(),
        };

        let games_mapping_json = match tokio::fs::read_to_string(&self.game_dir_mapping_file).await
        {
            Ok(game_dir_mapping) => {
                match serde_json::from_str::<serde_json::Value>(&game_dir_mapping) {
                    Ok(mapping) => mapping,
                    Err(error) => {
                        return Err(error::GameDirMappingError::UnexpectedFormat {
                            reason: error.to_string(),
                        });
                    }
                }
            }
            Err(error) => {
                return Err(error::GameDirMappingError::FileError {
                    reason: error.to_string(),
                })
            }
        };

        let games_mapping = match serde_json::from_value::<GameDirMapping>(games_mapping_json) {
            Ok(games_mapping) => games_mapping,
            Err(e) => {
                return Err(error::GameDirMappingError::UnexpectedFormat {
                    reason: e.to_string(),
                });
            }
        };

        // Validate contents
        for (game_path, game_config) in &games_mapping.0 {
            let game_cwd_path = game_host_server_games_dir.join(&game_config.cwd);
            let game_runnable_path = &game_cwd_path.join(&game_config.runnable);

            if !tokio::fs::try_exists(&game_cwd_path).await.unwrap() {
                return Err(error::GameDirMappingError::ConfigError {
                    reason: format!(
                        "Game cwd for '{}' ({}) does not exist.",
                        &game_path,
                        &game_cwd_path.to_string_lossy()
                    ),
                });
            }

            if !tokio::fs::try_exists(&game_runnable_path).await.unwrap() {
                return Err(error::GameDirMappingError::ConfigError {
                    reason: format!(
                        "Game runnable for '{}' ({}) does not exist.",
                        &game_path,
                        &game_runnable_path.to_string_lossy()
                    ),
                });
            }
        }

        // All good. Set new GameDirMapping
        self.game_dir_mapping = Arc::new(RwLock::new(games_mapping));

        Ok(())
    }
}
