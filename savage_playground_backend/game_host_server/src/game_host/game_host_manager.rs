use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use tokio::task::JoinHandle;

use crate::room_server::{
    client::ClientID,
    room::RoomID,
    server::{RoomServerHandle, RoomServerNotification},
};

use super::game_host::GameHost;

#[derive(Clone)]
pub struct GameHostManagerHandle {
    pub(self) server_handle: RoomServerHandle,
    pub(self) room_host_handle_map: Arc<RwLock<BTreeMap<RoomID, JoinHandle<()>>>>,
}

impl GameHostManagerHandle {
    pub fn new(server_handle: RoomServerHandle) -> GameHostManagerHandle {
        let room_host_handle_map: Arc<RwLock<BTreeMap<RoomID, JoinHandle<()>>>> =
            Default::default();

        let game_host_manager_handle = GameHostManagerHandle {
            server_handle,
            room_host_handle_map,
        };

        tokio::spawn(Self::server_notification_listener(
            game_host_manager_handle.clone(),
        ));

        game_host_manager_handle
    }

    async fn server_notification_listener(game_host_manager: GameHostManagerHandle) {
        let mut server_receiver = game_host_manager.server_handle.subscribe();

        while let Ok(notification) = server_receiver.recv().await {
            match notification {
                RoomServerNotification::RoomCreated { room_id, config, client_id } => {
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

    fn on_room_created(game_host_manager: GameHostManagerHandle, room_id: u64, config: serde_json::Value, owner_id: u64) {
        let task_handle = tokio::spawn(async move {
            let room_handle = game_host_manager.server_handle.get_room_handle(room_id);
            if room_handle.is_none() {
                return;
            }
    
            let room_handle = room_handle.unwrap();
    
            GameHost::new(config, owner_id, room_handle).serve().await;
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
}
