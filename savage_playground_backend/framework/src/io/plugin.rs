use bevy::prelude::{Plugin, First};

use super::{resource::PlayerInputManager, event::PlayerInputEvent, system::input_system};


#[derive(Default)]
pub struct IOPlugin;
impl Plugin for IOPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(PlayerInputManager::default())
            .add_event::<PlayerInputEvent>()
            .add_systems(First, input_system);
    }
}