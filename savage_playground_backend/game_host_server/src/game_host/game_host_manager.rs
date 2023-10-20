use std::{collections::BTreeMap, sync::{RwLock, Arc}};

use tokio::{task::JoinHandle};

use crate::server::{room::{RoomID}, server::{ServerHandle, ServerNotification}, client::ClientID};

use super::game_host::GameHost;


#[derive(Clone)]
pub struct GameHostManagerHandle {
    pub (self) server_handle: ServerHandle,
    pub (self) room_host_handle_map: Arc<RwLock<BTreeMap<RoomID, JoinHandle<()>>>>,
}

impl GameHostManagerHandle {
    pub fn new(server_handle: ServerHandle) -> GameHostManagerHandle {
        let room_host_handle_map: Arc<RwLock<BTreeMap<RoomID, JoinHandle<()>>>> = Default::default();

        let game_host_manager_handle = GameHostManagerHandle {
            server_handle,
            room_host_handle_map,
        };

        tokio::spawn(Self::server_notification_listener(game_host_manager_handle.clone()));

        game_host_manager_handle
    }

    async fn server_notification_listener(game_host_manager: GameHostManagerHandle) {
        let mut server_receiver = game_host_manager.server_handle.subscribe();

        while let Ok(notification) = server_receiver.recv().await {
            match notification {
                ServerNotification::RoomCreated { room_id, client_id } => {
                    let task_handle = tokio::spawn(Self::game_host_task(game_host_manager.clone(), client_id, room_id));
                    if let Ok(mut map_lock) = game_host_manager.room_host_handle_map.write() {
                        map_lock.insert(room_id, task_handle);
                    }
                },
                ServerNotification::RoomEmpty { room_id } => {
                    game_host_manager.server_handle.close_room(room_id);
                },
                ServerNotification::RoomClosed { room_id } => {
                    if let Ok(mut map_lock) = game_host_manager.room_host_handle_map.write() {
                        if let Some(host_handle) = map_lock.get(&room_id) {
                            host_handle.abort();
                        }
                        map_lock.remove(&room_id);
                    }
                },
            }
        }
    }

    async fn game_host_task(game_host_manager: GameHostManagerHandle, owner_id: ClientID, room_id: RoomID) {
        let room_handle = game_host_manager.server_handle.get_room_handle(room_id);
        if room_handle.is_none() {
            return;
        }

        let room_handle = room_handle.unwrap();

        GameHost::new(owner_id, room_handle).serve().await;
    }
}
