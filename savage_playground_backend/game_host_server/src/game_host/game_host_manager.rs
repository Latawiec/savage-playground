use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::room_server::{
    client::ClientID,
    room::RoomID,
    server::{RoomServerHandle, RoomServerNotification},
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
}

#[derive(Serialize, Deserialize, Clone)]
struct GameDirConfig {
    cwd: String,
    runnable: String,
}

#[derive(Serialize, Deserialize, Default)]
struct GameDirMapping(HashMap<String, GameDirConfig>);

#[derive(Clone)]
pub struct GameHostManagerHandle {
    pub(self) server_handle: RoomServerHandle,
    pub(self) room_host_handle_map: Arc<RwLock<BTreeMap<RoomID, JoinHandle<()>>>>,
    pub(self) game_dir_mapping: Arc<GameDirMapping>,
    pub(self) game_dir_mapping_file: PathBuf,
}

impl GameHostManagerHandle {
    pub fn new(server_handle: RoomServerHandle, game_dir_mapping_file: &Path) {
        let room_host_handle_map: Arc<RwLock<BTreeMap<RoomID, JoinHandle<()>>>> =
            Default::default();

        let game_host_manager_handle = GameHostManagerHandle {
            server_handle,
            room_host_handle_map,
            game_dir_mapping: Default::default(),
            game_dir_mapping_file: game_dir_mapping_file.to_owned(),
        };

        tokio::spawn(Self::server_notification_listener(
            game_host_manager_handle.clone(),
        ));
    }

    async fn server_notification_listener(mut game_host_manager: GameHostManagerHandle) {
        let mut server_receiver = game_host_manager.server_handle.subscribe();
        if let Err(error) = game_host_manager.load_game_dir_mapping().await {
            tracing::error!("Failed to load game dir mapping: {:?}", error);
        }

        while let Ok(notification) = server_receiver.recv().await {
            match notification {
                RoomServerNotification::RoomCreated {
                    room_id,
                    config,
                    client_id,
                } => {
                    Self::on_room_created(game_host_manager.clone(), room_id, config, client_id);
                }
                RoomServerNotification::RoomEmpty { room_id } => {
                    Self::on_room_empty(game_host_manager.clone(), room_id);
                }
                RoomServerNotification::RoomClosed { room_id } => {
                    Self::on_room_closed(game_host_manager.clone(), room_id);
                }
            }
        }
    }

    fn on_room_created(
        game_host_manager: GameHostManagerHandle,
        room_id: u64,
        config: serde_json::Value,
        owner_id: u64,
    ) {
        let game_config = serde_json::from_value::<
            room_server_interface::schema::game_config::GameConfig,
        >(config);

        if game_config.is_err() {
            tracing::error!(
                "Received invalid game config: {}",
                game_config.err().unwrap()
            );
            return;
        }
        let game_config = game_config.unwrap();
        let game_dir_config = game_host_manager.game_dir_mapping.0.get(&game_config.game_path.unwrap());

        if let None = &game_dir_config {
            // tracing::error!("Requested game {} not registered.", game_config.game_path.);
            return;
        }
        let game_dir_config = game_dir_config.unwrap().to_owned();

        let task_handle = tokio::spawn(async move {
            let room_handle = game_host_manager.server_handle.get_room_handle(room_id);
            if room_handle.is_none() {
                return;
            }

            let room_handle = room_handle.unwrap();

            let mut game_host = GameHost::new(owner_id, room_handle, PathBuf::from(game_dir_config.cwd), PathBuf::from(game_dir_config.runnable));
            game_host.run().await;
        });
        if let Ok(mut map_lock) = game_host_manager.room_host_handle_map.write() {
            map_lock.insert(room_id, task_handle);
        }
    }

    fn on_room_empty(game_host_manager: GameHostManagerHandle, room_id: u64) {
        game_host_manager.server_handle.close_room(room_id);
    }

    fn on_room_closed(game_host_manager: GameHostManagerHandle, room_id: u64) {
        if let Ok(mut map_lock) = game_host_manager.room_host_handle_map.write() {
            if let Some(host_handle) = map_lock.get(&room_id) {
                host_handle.abort();
            }
            map_lock.remove(&room_id);
        }
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
        self.game_dir_mapping = Arc::new(games_mapping);

        Ok(())
    }
}
