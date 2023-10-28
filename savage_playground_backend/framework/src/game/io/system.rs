use bevy::prelude::{EventReader, ResMut};

use crate::io::event::GameInputMessage;

use super::{event::PlayerInputEvent, resource::PlayerInputManager};


pub fn raw_input_parse_system(
    mut ev_input_message: EventReader<GameInputMessage>
) {
    for msg in ev_input_message.iter() {
        let str_msg = String::from_utf8_lossy(&msg.0);
        tracing::info!("Input: {:?}", str_msg);
    }
}

pub fn input_register_system(
    mut ev_player_input: EventReader<PlayerInputEvent>,
    mut res_player_input: ResMut<PlayerInputManager>,
) {
    for ev in ev_player_input.iter() {
        res_player_input.set_input_state(ev.player_id, ev.new_state);
    }
}
