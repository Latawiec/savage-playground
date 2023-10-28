use std::collections::BTreeMap;

use bevy::prelude::Resource;

use crate::game::common::player_type::PlayerID;

use super::input::{InputState, InputStateFlags};

#[derive(Default, Resource)]
pub struct PlayerInputManager {
    players_input: BTreeMap<PlayerID, InputState>,
}

impl PlayerInputManager {
    pub fn set_input_state(&mut self, player_id: PlayerID, new_state: InputStateFlags) {
        self.players_input
            .entry(player_id)
            .and_modify(|current_state| current_state.apply_state(new_state))
            .or_insert(InputState::from_state(new_state));
    }

    pub fn get_input_state(&self, player_id: &PlayerID) -> Option<&InputState> {
        self.players_input.get(player_id)
    }
}
