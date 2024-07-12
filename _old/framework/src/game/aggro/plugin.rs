use bevy::prelude::{Update, Plugin};

use super::{system::aggro_system, event::AggroChangeEvent};


#[derive(Default)]
pub struct AggroPlugin;
impl Plugin for AggroPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<AggroChangeEvent>()
            .add_systems(Update, aggro_system);
    }
}