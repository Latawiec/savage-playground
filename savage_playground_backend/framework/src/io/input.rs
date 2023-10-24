pub type KeyFlag = u32;
pub type InputStateFlags = KeyFlag; // 1 on keys pushed down. 0 on the rest.
pub type InputDiffFlags = KeyFlag; // 1 on each field that has changed. 0 on the rest.

#[derive(Clone, Copy)]
pub enum State {
    Up,
    Down,
    Pressed,
    Released,
}

#[derive(Default, Debug)]
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
        self.state & key as InputStateFlags != 0
    }
    pub fn is_key_up(&self, key: KeyFlag) -> bool {
        self.state & key as InputStateFlags == 0
    }
    pub fn is_just_pressed(&self, key: KeyFlag) -> bool {
        self.is_key_down(key) && (self.changed & key as InputDiffFlags != 0)
    }
    pub fn is_just_released(&self, key: KeyFlag) -> bool {
        self.is_key_up(key) && (self.changed & key as InputDiffFlags != 0)
    }
    pub fn changed(&self) -> bool {
        self.changed != 0
    }
    pub fn apply_state(&mut self, new_state: InputStateFlags) {
        self.changed = self.state ^ new_state;
        self.state = new_state;
    }
}