use bevy::prelude::{Plugin, Update, PostUpdate};

use super::{event::PlayerInputEvent, resource::PlayerInputManager, system::{input_register_system, raw_input_parse_system}};

#[derive(Default)]
pub struct GameIOPlugin;
impl Plugin for GameIOPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerInputManager::default())
            .add_event::<PlayerInputEvent>()
            .add_systems(Update, input_register_system)
            .add_systems(PostUpdate, raw_input_parse_system);
    }
}
