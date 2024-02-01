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

struct RoomHostDataMapping {
    room_clients_map: BTreeMap<RoomHandle, BTreeSet<ClientHandle>>,
    client_rooms_map: BTreeMap<ClientHandle, BTreeSet<RoomHandle>>,
}

pub struct RoomHostState {
    room_host_data: RwLock<RoomHostDataMapping>,
    broadcast_sender: tokio::sync::broadcast::Sender<RoomHostNotification>,
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

    fn subscribe_host_info(
        &self,
    ) -> Result<tokio::sync::broadcast::Receiver<RoomHostNotification>, RoomHostError> {
        Ok(self.broadcast_sender.subscribe())
    }
}

impl RoomHostManagement for RoomHostState {
    async fn create_client(&mut self) -> ClientHandle {
        ClientHandle(
            self.client_handle_monotonic_counter
                .fetch_add(1, Ordering::Relaxed),
        )
    }

    async fn create_room(&mut self) -> RoomHandle {
        RoomHandle(
            self.room_handle_monotonic_counter
                .fetch_add(1, Ordering::Relaxed),
        )
    }

    async fn remove_client(&mut self, client: ClientHandle) -> Result<(), RoomHostError> {
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

        // Then remove from clients map.
        if let None = lock.client_rooms_map.remove(&client) {
            return Err(RoomHostError::StatePoisoned);
        }

        // Unlock
        drop(lock);

        // Notify
        for room in client_rooms {
            self.broadcast_sender
                .send(RoomHostNotification::ClientLeft { room, client });
        }
        self.broadcast_sender
            .send(RoomHostNotification::ClientRemoved { client });

        Ok(())
    }

    async fn update_room(
        &mut self,
        _client: ClientHandle,
        _room: RoomHandle,
    ) -> Result<(), RoomHostError> {
        todo!()
    }

    async fn remove_room(
        &mut self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError> {
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
            self.broadcast_sender
                .send(RoomHostNotification::ClientLeft { room, client });
        }
        self.broadcast_sender
            .send(RoomHostNotification::RoomDestroyed { room });

        Ok(())
    }

    async fn join_room(
        &mut self,
        client: ClientHandle,
        room: RoomHandle,
    ) -> Result<(), RoomHostError> {
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

        self.broadcast_sender.send(RoomHostNotification::ClientJoined { room, client });
        Ok(())
    }

    async fn leave_room(
        &mut self,
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

        if let Some(client_rooms) = lock.client_rooms_map.get_mut(&client) {
            client_rooms.remove(&room);
        }

        self.broadcast_sender.send(RoomHostNotification::ClientLeft { room, client });
        Ok(())
    }
}
