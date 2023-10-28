use bevy::prelude::{EventReader, ResMut};

use super::{event::PlayerInputEvent, resource::PlayerInputManager};

pub fn input_register_system(
    mut ev_player_input: EventReader<PlayerInputEvent>,
    mut res_player_input: ResMut<PlayerInputManager>,
) {
    for ev in ev_player_input.iter() {
        res_player_input.set_input_state(ev.player_id, ev.new_state);
    }
}
