use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Settings {
    pub resolution: [u32; 2]
}