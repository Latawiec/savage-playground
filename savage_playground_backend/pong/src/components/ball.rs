use bevy::prelude::Component;

#[derive(Component)]
pub struct Ball {
    pub speed: f32,
    pub velocity: [f32; 2]
}