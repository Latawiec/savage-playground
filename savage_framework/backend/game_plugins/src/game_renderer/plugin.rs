use bevy::app::{Plugin, PostUpdate};

use super::{event::SceneElementsBatchEvent, system::scene_full_update_system};

#[derive(Default)]
pub struct GameRendererPlugin;
impl Plugin for GameRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<SceneElementsBatchEvent>()
            .add_systems(PostUpdate, scene_full_update_system);
    }
}
