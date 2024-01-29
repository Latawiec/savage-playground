use std::collections::HashMap;

use super::{
    client::ClientID,
    room::{RoomHandle, RoomID},
};

pub trait RoomServerBackend {
    fn create_room(
        &self,
        client_id: ClientID,
        room_handle: RoomHandle,
    ) -> Result<(), warp::Rejection>;

    fn update_room(
        &self,
        client_id: ClientID,
        room_handle: RoomHandle,
        config: serde_json::Value,
    ) -> Result<(), warp::Rejection>;

    fn join_room(
        &self,
        client_id: ClientID,
        room_handle: RoomHandle,
    ) -> Result<(), warp::Rejection>;
    
    fn leave_room(
        &self,
        client_id: ClientID,
        room_handle: RoomHandle,
    ) -> Result<(), warp::Rejection>;
}
