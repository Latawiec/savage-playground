use bevy::prelude::{Plugin, PostUpdate};

use super::system::self_destruct_system;

#[derive(Default)]
pub struct SelfDestructPlugin;
impl Plugin for SelfDestructPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, self_destruct_system);
    }
}
