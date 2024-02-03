use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicU64, Ordering},
        RwLock,
    },
};

use crate::room_host::{
    traits::{room_host_info::RoomHostInfo, room_host_management::RoomHostManagement},
    types::{
        client_handle::ClientHandle, error::RoomHostError, room_handle::RoomHandle,
        room_host_notification::RoomHostNotification,
    },
};

#[derive(Default)]
struct RoomHostDataMapping {
    room_clients_map: BTreeMap<RoomHandle, BTreeSet<ClientHandle>>,
    client_rooms_map: BTreeMap<ClientHandle, BTreeSet<RoomHandle>>,
}

pub struct RoomHostState {
    room_host_data: RwLock<RoomHostDataMapping>,
    notification_sender: tokio::sync::broadcast::Sender<RoomHostNotification>,
    client_handle_monotonic_counter: AtomicU64,
    room_handle_monotonic_counter: AtomicU64,
}

impl RoomHostState {
    fn data_read_lock(
        &self,
    ) -> Result<std::sync::RwLockReadGuard<'_, RoomHostDataMapping>, RoomHostError> {
        let lock = self.room_host_data.read();
        if let Err(e) = &lock {
            return Err(RoomHostError::InternalError {
                message: format!("Lock poisoned: {}", e.to_string()),
            });
        }
        Ok(lock.unwrap())
    }

    fn data_write_lock(
        &self,
    ) -> Result<std::sync::RwLockWriteGuard<'_, RoomHostDataMapping>, RoomHostError> {
        let lock = self.room_host_data.write();
        if let Err(e) = &lock {
            return Err(RoomHostError::InternalError {
                message: format!("Lock poisoned: {}", e.to_string()),
            });
        }
        Ok(lock.unwrap())
    }
}

impl Default for RoomHostState {
    fn default() -> Self {
        const DEFAULT_BROADCAST_QUEUE_SIZE: usize = 1024;
        let (notification_sender, _) =
            tokio::sync::broadcast::channel(DEFAULT_BROADCAST_QUEUE_SIZE);
        Self {
            room_host_data: Default::default(),
            notification_sender,
            client_handle_monotonic_counter: Default::default(),
            room_handle_monotonic_counter: Default::default(),
        }
    }
}

impl RoomHostInfo for RoomHostState {
    async fn get_room_clients(
        &self,
        room: RoomHandle,
    ) -> Result<BTreeSet<ClientHandle>, RoomHostError> {
        let lock = self.data_read_lock()?;

        if let Some(clients) = lock.room_clients_map.get(&room) {
            return Ok(clients.clone());
        } else {
            return Err(RoomHostError::RoomNotFound);
        }
    }

    async fn get_rooms(&self) -> Result<BTreeSet<RoomHandle>, RoomHostError> {
        let lock = self.data_read_lock()?;
        Ok(lock.room_clients_map.keys().copied().collect())
    }

    async fn get_client_rooms(
        &self,
        client: ClientHandle,
    ) -> Result<BTreeSet<RoomHandle>, RoomHostError> {
        let lock = self.data_read_lock()?;

        if let Some(rooms) = lock.client_rooms_map.get(&client) {
            return Ok(rooms.clone());
        } else {
            return Err(RoomHostError::ClientNotFound);
        }
    }

    async fn get_clients(&self) -> Result<BTreeSet<ClientHandle>, RoomHostError> {
        let lock = self.data_read_lock()?;
        Ok(lock.client_rooms_map.keys().copied().collect())
    }

    fn subscribe_host_info(
        &self,
    ) -> Result<tokio::sync::broadcast::Receiver<RoomHostNotification>, RoomHostError> {
        Ok(self.notification_sender.subscribe())
    }
}

impl RoomHostManagement for RoomHostState {
    async fn create_client(&self) -> Result<ClientHandle, RoomHostError> {
        let client = ClientHandle(
            self.client_handle_monotonic_counter
                .fetch_add(1, Ordering::Relaxed),
        );

        let mut lock = self.data_write_lock()?;
        if let Some(_) = lock.client_rooms_map.insert(client, Default::default()) {
            return Err(RoomHostError::StatePoisoned);
        }
        drop(lock);

        let _ = self
            .notification_sender
            .send(RoomHostNotification::ClientCreated { client });
        Ok(client)
    }

    async fn create_room(&self) -> Result<RoomHandle, RoomHostError> {
        let room = RoomHandle(
            self.room_handle_monotonic_counter
                .fetch_add(1, Ordering::Relaxed),
        );

        let mut lock = self.data_write_lock()?;
        if let Some(_) = lock.room_clients_map.insert(room, Default::default()) {
            return Err(RoomHostError::StatePoisoned);
        }
        drop(lock);

        let _ = self
            .notification_sender
            .send(RoomHostNotification::RoomCreated { room });
        Ok(room)
    }

    async fn remove_client(&self, client: ClientHandle) -> Result<(), RoomHostError> {
        let mut lock = self.data_write_lock()?;

        let client_rooms = lock.client_rooms_map.remove(&client);
        if let None = &client_rooms {
            return Err(RoomHostError::ClientNotFound);
        }
        let client_rooms = client_rooms.unwrap();

        // Remove client from rooms.
        for room in &client_rooms {
            if let Some(clients) = lock.room_clients_map.get_mut(room) {
                if !clients.remove(&client) {
                    return Err(RoomHostError::StatePoisoned);
                }
            } else {
                return Err(RoomHostError::StatePoisoned);
            }
        }

        // Unlock
        drop(lock);

        // Notify
        for room in client_rooms {
            let _ = self
                .notification_sender
                .send(RoomHostNotification::ClientLeft { room, client });
        }
        let _ = self
            .notification_sender
            .send(RoomHostNotification::ClientRemoved { client });

        Ok(())
    }

    async fn remove_room(&self, room: RoomHandle) -> Result<(), RoomHostError> {
        let mut lock = self.data_write_lock()?;

        let clients = lock.room_clients_map.remove(&room);
        if let None = &clients {
            return Err(RoomHostError::RoomNotFound);
        }
        let clients = clients.unwrap();

        for client in &clients {
            if let Some(client_rooms) = lock.client_rooms_map.get_mut(client) {
                if !client_rooms.remove(&room) {
                    return Err(RoomHostError::StatePoisoned);
                }
            } else {
                return Err(RoomHostError::StatePoisoned);
            }
        }

        // Unlock
        drop(lock);

        for client in clients {
            let _ = self
                .notification_sender
                .send(RoomHostNotification::ClientLeft { room, client });
        }
        let _ = self
            .notification_sender
            .send(RoomHostNotification::RoomDestroyed { room });

        Ok(())
    }

    async fn join_room(&self, client: ClientHandle, room: RoomHandle) -> Result<(), RoomHostError> {
        let mut lock = self.data_write_lock()?;

        let opt_room_clients = lock.room_clients_map.get_mut(&room);
        if let None = &opt_room_clients {
            return Err(RoomHostError::RoomNotFound);
        }
        let room_clients = opt_room_clients.unwrap();

        if !room_clients.insert(client) {
            return Err(RoomHostError::ClientAlreadyInRoom);
        }

        if let Some(client_rooms) = lock.client_rooms_map.get_mut(&client) {
            client_rooms.insert(room);
        }
        drop(lock);

        let _ = self
            .notification_sender
            .send(RoomHostNotification::ClientJoined { room, client });
        Ok(())
    }

    async fn leave_room(
        &self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError> {
        let mut lock = self.data_write_lock()?;

        let opt_room_clients = lock.room_clients_map.get_mut(&room);
        if let None = &opt_room_clients {
            return Err(RoomHostError::RoomNotFound);
        }
        let room_clients = opt_room_clients.unwrap();

        if !room_clients.remove(&client) {
            return Err(RoomHostError::ClientNotInRoom);
        }
        let is_room_empty = room_clients.is_empty();

        if let Some(client_rooms) = lock.client_rooms_map.get_mut(&client) {
            client_rooms.remove(&room);
        }
        drop(lock);

        let _ = self
            .notification_sender
            .send(RoomHostNotification::ClientLeft { room, client });

        if is_room_empty {
            let _ = self
                .notification_sender
                .send(RoomHostNotification::RoomEmpty { room });
        }
        Ok(())
    }
}
