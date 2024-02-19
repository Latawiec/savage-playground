use std::{borrow::Borrow, collections::BTreeMap, sync::{Arc, RwLock}};

use rocket_ws::stream::DuplexStream;
use room_server_interface::schema::game_config::GameConfig;

use crate::{game_launcher::{self, game_instance, game_launcher::GameLauncher}, server::connection::client_connection::ConnectionCloseHandle};

use super::{game_room::GameRoom, handle_gen::HandleGenerator, types::RoomHandle};




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

    pub fn create_room(&self, game_config: GameConfig, game_launcher: &GameLauncher) -> Option<RoomHandle> {
        let room_handle = self.room_handle_gen.next();
        let game_id = game_config.game_id.as_ref()?;
        let game_instance = game_launcher.launch_game(game_id, &vec![]);
        if let Ok(game_instance) = game_instance {
            let game_room = GameRoom::new(room_handle, game_instance, game_config);
            if let Ok(mut wlock) = self.game_rooms.write() {
                let _ = wlock.insert(room_handle, game_room);
            }
        }

        Some(room_handle)
    }

    pub fn join_room(&self, room_handle: RoomHandle, ws_stream: DuplexStream) -> Option<ConnectionCloseHandle> {
        if let Ok(rlock) = self.game_rooms.read() {
            if let Some(room) = rlock.get(&room_handle) {
                return Some(room.connect(ws_stream));
            }
        }
        None
    }
}