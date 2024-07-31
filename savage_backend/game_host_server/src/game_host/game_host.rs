use rocket_ws::stream::DuplexStream;
use tracing::info_span;
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use super::{
    game_room::{disconnect_reason::GameRoomDisconnectReason, game_room::GameRoom},
    handle_gen::HandleGenerator,
    interface::schema::game_config::GameConfig,
    types::RoomHandle,
};
use crate::game_launcher::game_launcher::GameLauncher;

pub struct GameHost {
    game_rooms: Arc<RwLock<BTreeMap<RoomHandle, GameRoom>>>,
    room_handle_gen: HandleGenerator<RoomHandle>,
}

impl GameHost {
    pub fn new() -> GameHost {
        GameHost {
            game_rooms: Default::default(),
            room_handle_gen: Default::default(),
        }
    }

    pub fn create_room(
        &self,
        game_config: GameConfig,
        game_launcher: &GameLauncher,
    ) -> Option<RoomHandle> {
        let room_handle = self.room_handle_gen.next();
        let game_id = game_config.game_id.clone()?;
        let game_instance = game_launcher.launch_game(&game_id, &["--id", &room_handle.0.to_string()]);

        if let Err(game_launcher_error) = game_instance {
            use crate::game_launcher::error::GameLauncherError;

            let error_msg = match game_launcher_error {
                GameLauncherError::GameMappingFileError { reason } => {
                    format!("game launch failed: {}", reason)
                }
                GameLauncherError::GameMappingFileIllFormed { reason } => {
                    format!("game launch failed: {}", reason)
                }
                GameLauncherError::GameNotFound { game_id: _ } => {
                    format!("game launch failed: game not found")
                }
                GameLauncherError::InstanceStartupError { reason } => {
                    format!("game launch failed: {}", reason)
                }
                GameLauncherError::InstanceKillError { reason } => {
                    format!("game launch failed: {}", reason)
                }
            };
            tracing::error!(name: "create_room", target: "game_host", room_id = room_handle.0, game_id, "{}", error_msg);
            return None;
        }

        let game_instance = game_instance.unwrap();
        let game_room_span = info_span!("game_room", room_id = room_handle.0, game_id);
        let game_room = GameRoom::new(game_instance, game_config, game_room_span);
        match self.game_rooms.write() {
            Ok(mut wlock) => {
                let _ = wlock.insert(room_handle, game_room);
            }
            Err(err) => {
                tracing::error!(name: "create_room", target: "game_host", room_id = room_handle.0, "rwlock error: {}", err);
                return None;
            }
        };

        tracing::info!(name: "create_room", target: "game_host", room_id = room_handle.0, game_id, "created");
        Some(room_handle)
    }

    pub async fn join_room(
        &self,
        room_handle: RoomHandle,
        ws_stream: DuplexStream,
    ) -> GameRoomDisconnectReason {
        let game_room_connection_handle = match self.game_rooms.read() {
            Ok(rlock) => match rlock.get(&room_handle) {
                Some(room) => Some(room.connect(ws_stream)),
                None => return GameRoomDisconnectReason::RoomDoesNotExist,
            },
            Err(err) => {
                tracing::error!(name: "join_room", target: "game_host", room_id = room_handle.0, "rwlock error: {}", err);
                return GameRoomDisconnectReason::UnexpectedError(err.to_string());
            }
        };
        let game_room_connection_handle = game_room_connection_handle.unwrap();

        tracing::info!(name: "join_room", target: "game_host", room_id = room_handle.0, client_id = game_room_connection_handle.client_id(), "connected");
        let disconnect_reason = game_room_connection_handle.wait().await;
        tracing::info!(name: "join_room", target: "game_host", room_id = room_handle.0, client_id = game_room_connection_handle.client_id(), "disconnected: {}", disconnect_reason);

        return disconnect_reason;
    }

    pub fn delete_room(&self, room_handle: RoomHandle) -> Option<()> {
        match self.game_rooms.write() {
            Ok(mut wlock) => match wlock.remove(&room_handle) {
                Some(game_room) => {
                    drop(game_room);
                    tracing::info!(name: "delete_room", target: "game_host", room_id = room_handle.0, "room deleted");
                    return Some(());
                }
                None => {
                    tracing::warn!(name: "delete_room", target: "game_host", room_id = room_handle.0, "couldn't delete room - room not found");
                    return None;
                }
            },
            Err(err) => {
                tracing::error!(name: "delete_room", target: "game_host", room_id = room_handle.0, "rwlock error: {}", err);
                return None;
            }
        }
    }
}
