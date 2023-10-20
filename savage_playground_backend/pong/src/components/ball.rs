use bevy::prelude::Component;

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right
}

#[derive(Component)]
pub struct Ball {
    pub speed: f32,
    pub direction: Direction
}