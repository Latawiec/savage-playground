use bevy::prelude::{Plugin, Update};

use super::system::{player_motion_input_system, player_sprite_update_system};

#[derive(Default)]
pub struct PlayerSystemsPlugin;
impl Plugin for PlayerSystemsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_motion_input_system)
            .add_systems(Update, player_sprite_update_system);
    }
}
