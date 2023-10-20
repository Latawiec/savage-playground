use bevy::prelude::{Component, KeyCode};

pub enum Side {
    Left,
    Right
}

#[derive(Component)]
pub struct Palette {
    pub side: Side,
    pub up_key: KeyCode,
    pub down_key: KeyCode
}