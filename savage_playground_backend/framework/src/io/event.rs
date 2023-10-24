use bevy::prelude::Event;

use super::input::InputStateFlags;
use crate::types::player::PlayerID;

#[derive(Event)]
pub struct PlayerInputEvent {
    pub player_id: PlayerID,
    pub new_state: InputStateFlags,
    pub timestamp: std::time::Duration, // Since Epoch of course.
}
