use std::sync::Arc;

use super::{room_host_messenger::RoomHostMessenger, room_host_state::RoomHostState, traits::{room_host_communication::{RoomHostInput, RoomHostOutput}, room_host_info::RoomHostInfo, room_host_management::RoomHostManagement}};

#[derive(Default, Clone)]
pub struct RoomHost {
    messenger: Arc<RoomHostMessenger>,
    state: Arc<RoomHostState>,
}

impl RoomHost {
    pub fn get_client_messenger(&self) -> Arc<impl RoomHostInput + Send + Sync> {
        self.messenger.clone()
    }

    pub fn get_host_messenger(&self) -> Arc<impl RoomHostOutput + Send + Sync> {
        self.messenger.clone()
    }

    pub fn get_host_manager(&self) -> Arc<impl RoomHostManagement + Send + Sync> {
        self.state.clone()
    }

    pub fn get_host_info(&self) -> Arc<impl RoomHostInfo + Send + Sync> {
        self.state.clone()
    }
}