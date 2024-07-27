use bevy::prelude::Event;

use crate::proto;

#[derive(Event)]
pub struct FFXIVGameInputEvent{
    pub player_id: u64,
    pub input_data: proto::ffxiv_toolkit::FfxivGameInput,
}
