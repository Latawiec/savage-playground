use std::{
    sync::mpsc::SyncSender,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::prelude::{Input, KeyCode, Res, Resource};

use crate::{
    resources::io::input::{InputStateFlags, KeyFlag, NewInput},
    types::player::PlayerID,
};

#[derive(Resource)]
pub struct LocalInput {
    sender: SyncSender<NewInput>,
    pub player_id: PlayerID,
}

impl LocalInput {
    pub fn new(player_id: PlayerID, sender: SyncSender<NewInput>) -> LocalInput {
        LocalInput { sender, player_id }
    }
    pub fn new_input(&self, input: NewInput) {
        self.sender
            .send(input)
            .expect("Couldn't send debug local input");
    }
}

pub fn local_input_system(keys: Res<Input<KeyCode>>, local_input: Res<LocalInput>) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let mut keys_state = InputStateFlags::default();
    let mut changed = false;


    if !(keys.just_released(KeyCode::W) || keys.just_released(KeyCode::A) || keys.just_released(KeyCode::S) || keys.just_released(KeyCode::D) ||
         keys.just_pressed(KeyCode::W)  || keys.just_pressed(KeyCode::A)  || keys.just_pressed(KeyCode::S)  || keys.just_pressed(KeyCode::D)) {
        return;
    }

    if keys.pressed(KeyCode::W) {
        keys_state |= KeyFlag::Up as InputStateFlags;
        changed = true;
    }
    if keys.pressed(KeyCode::A) {
        keys_state |= KeyFlag::Left as InputStateFlags;
        changed = true;
    }
    if keys.pressed(KeyCode::S) {
        keys_state |= KeyFlag::Down as InputStateFlags;
        changed = true;
    }
    if keys.pressed(KeyCode::D) {
        keys_state |= KeyFlag::Right as InputStateFlags;
        changed = true;
    }

    let mut new_input = NewInput::default();
    new_input.timestamp = since_the_epoch;
    new_input.player_id = local_input.player_id;
    new_input.new_state = keys_state;

    local_input.new_input(new_input);
}
