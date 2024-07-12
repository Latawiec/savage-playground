use bevy::prelude::{Resource, Vec2};

#[derive(Resource)]
pub struct EnvironmentConfig {
    pub environment_scale: f32,
    pub movement_speed: f32,
    /// If the player presses "Up" button, where is Up?
    /// This field maps Up to an in-world vector for movement.
    pub forward: Vec2,
    pub right: Vec2,
}
