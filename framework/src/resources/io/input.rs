use std::{
    collections::BTreeMap,
    sync::{mpsc::Receiver, Mutex},
};

use bevy::prelude::Resource;

use crate::types::player::PlayerID;

pub type InputStateFlags = u32; // 1 on keys pushed down. 0 on the rest.
pub type InputDiffFlags = u32; // 1 on each field that has changed. 0 on the rest.

#[derive(Clone, Copy)]
pub enum KeyFlag {
    Up = 1 << 1,
    Down = 1 << 2,
    Left = 1 << 3,
    Right = 1 << 4,
    Spell1 = 1 << 5,
    Spell2 = 1 << 6,
    Spell3 = 1 << 7,
}

#[derive(Clone, Copy)]
pub enum State {
    Up,
    Down,
    Pressed,
    Released,
}

#[derive(Default)]
pub struct NewInput {
    pub player_id: PlayerID,
    pub new_state: InputStateFlags,
    pub timestamp: std::time::Duration, // Since Epoch of course.
}

#[derive(Default)]
pub struct InputState {
    state: InputStateFlags,
    changed: InputDiffFlags,
}

impl InputState {
    pub fn from_state(state: InputStateFlags) -> InputState {
        InputState {
            state,
            changed: state,
        }
    }
    pub fn is_key_down(&self, key: KeyFlag) -> bool {
        self.state & key as InputStateFlags == 1
    }
    pub fn is_key_up(&self, key: KeyFlag) -> bool {
        self.state & key as InputStateFlags == 0
    }
    pub fn is_just_pressed(&self, key: KeyFlag) -> bool {
        self.is_key_down(key) && (self.changed & key as InputDiffFlags == 1)
    }
    pub fn is_just_released(&self, key: KeyFlag) -> bool {
        self.is_key_up(key) && (self.changed & key as InputDiffFlags == 1)
    }
    pub fn changed(&self) -> bool {
        self.changed == 0
    }
    pub fn apply_state(&mut self, new_state: InputStateFlags) {
        self.changed = self.state ^ new_state;
        self.state = new_state;
    }
}

#[derive(Resource)]
pub struct InputManager {
    receiver: Mutex<Receiver<NewInput>>,
    pub players_input: BTreeMap<PlayerID, InputState>,
}

impl InputManager {
    pub fn new(input_receiver: Receiver<NewInput>) -> Self {
        InputManager {
            receiver: Mutex::new(input_receiver),
            players_input: Default::default(),
        }
    }

    pub fn process_input(&mut self) -> bool {
        #[cfg(debug_assertions)]
        {
            const MAX_PLAYERS_COUNT: usize = 8;
            if self.players_input.len() > MAX_PLAYERS_COUNT {
                tracing::warn!(
                    "Number of input contributors ({}) exceeds expected value ({})",
                    self.players_input.len(),
                    MAX_PLAYERS_COUNT,
                )
            }
        }

        match self.receiver.lock() {
            Ok(receiver) => {
                for input in receiver.try_iter() {
                    self.players_input
                        .entry(input.player_id)
                        .and_modify(|state| state.apply_state(input.new_state))
                        .or_insert(InputState::from_state(input.new_state));
                }
                true
            }
            Err(err) => {
                tracing::error!(
                    "Error trying to lock InputManager receiver. {:?}",
                    err.get_ref()
                );
                false
            }
        }
    }
}
