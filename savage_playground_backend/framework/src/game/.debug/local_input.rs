use std::time::{SystemTime, UNIX_EPOCH};

use bevy::prelude::{EventWriter, Input, KeyCode, Plugin, PreUpdate, Res, Resource};

use crate::{
    game::common::player_type::PlayerID,
    io::{default_key_flags::DefaultKeyFlags, event::PlayerInputEvent, input::InputStateFlags},
};

#[derive(Resource)]
pub struct LocalPlayerInput {
    pub player_id: PlayerID,
}

pub fn local_player_input_system(
    mut ev_player_input: EventWriter<PlayerInputEvent>,
    keys: Res<Input<KeyCode>>,
    res_local_player_input: Option<Res<LocalPlayerInput>>,
) {
    if res_local_player_input.is_none() {
        return;
    }
    let res_local_player_input = res_local_player_input.unwrap();
    
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let mut keys_state = InputStateFlags::default();

    if !(keys.just_released(KeyCode::W)
        || keys.just_released(KeyCode::A)
        || keys.just_released(KeyCode::S)
        || keys.just_released(KeyCode::D)
        || keys.just_pressed(KeyCode::W)
        || keys.just_pressed(KeyCode::A)
        || keys.just_pressed(KeyCode::S)
        || keys.just_pressed(KeyCode::D))
    {
        return;
    }

    if keys.pressed(KeyCode::W) {
        keys_state |= DefaultKeyFlags::Up as InputStateFlags;
    }
    if keys.pressed(KeyCode::A) {
        keys_state |= DefaultKeyFlags::Left as InputStateFlags;
    }
    if keys.pressed(KeyCode::S) {
        keys_state |= DefaultKeyFlags::Down as InputStateFlags;
    }
    if keys.pressed(KeyCode::D) {
        keys_state |= DefaultKeyFlags::Right as InputStateFlags;
    }

    let input_event = PlayerInputEvent {
        timestamp: since_the_epoch,
        player_id: res_local_player_input.player_id,
        new_state: keys_state,
    };

    ev_player_input.send(input_event);
}

#[derive(Default)]
pub struct LocalInputPlugin;
impl Plugin for LocalInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, local_player_input_system);
    }
}
