use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use rocket_ws::stream::DuplexStream;
use room_server_interface::schema::game_config::GameConfig;

use super::{
    disconnect_reason::DisconnectReason, game_room::GameRoom, handle_gen::HandleGenerator,
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
        let game_id = game_config.game_id.as_ref()?;
        let game_instance = game_launcher.launch_game(game_id, &vec![]);
        if let Ok(game_instance) = game_instance {
            let game_room = GameRoom::new(game_instance, game_config);
            if let Ok(mut wlock) = self.game_rooms.write() {
                let _ = wlock.insert(room_handle, game_room);
            }
        }

        Some(room_handle)
    }

    pub async fn join_room(
        &self,
        room_handle: RoomHandle,
        ws_stream: DuplexStream,
    ) -> DisconnectReason {
        let connect_join_handle = match self.game_rooms.read() {
            Ok(rlock) => match rlock.get(&room_handle) {
                Some(room) => Some(room.connect(ws_stream)),
                None => None,
            },
            Err(_) => None,
        };

        if let Some(connect_join_handle) = connect_join_handle {
            match connect_join_handle.await {
                Ok(disconnect_reason) => return disconnect_reason,
                Err(err) => DisconnectReason::UnexpectedError(err.to_string()),
            }
        } else {
            DisconnectReason::RoomDoesNotExist
        }
    }

    pub fn delete_room(&self, room_handle: RoomHandle) -> Option<()> {
        if let Ok(mut wlock) = self.game_rooms.write() {
            let _ = wlock.remove(&room_handle);
            return Some(());
        }
        None
    }
}
