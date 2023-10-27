use bevy::prelude::Event;

use crate::game::common::player_type::PlayerID;

use super::input::InputStateFlags;

#[derive(Event)]
pub struct PlayerInputEvent {
    pub player_id: PlayerID,
    pub new_state: InputStateFlags,
    pub timestamp: std::time::Duration, // Since Epoch of course.
}
