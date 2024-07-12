use bevy::prelude::{Last, Plugin};

use super::{
    event::{GameInputMessage, GameOutputMessage},
    system::io_exchange_system,
};

#[derive(Default)]
pub struct IOPlugin;
impl Plugin for IOPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<GameInputMessage>()
            .add_event::<GameOutputMessage>()
            .add_systems(Last, io_exchange_system);
    }
}
