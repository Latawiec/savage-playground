use std::collections::BTreeMap;

use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MovementDirection {
    Forward,
    Back,
    Left,
    Right,
}

/// Applies appropriate sprite tile based on player's movement direction.
#[derive(Component)]
pub struct PlayerSprite {
    dir_sprite_map: BTreeMap<MovementDirection, (u32, u32)>,
}

impl PlayerSprite {
    pub fn get_selection(&self, dir: MovementDirection) -> Option<(u32, u32)> {
        self.dir_sprite_map.get(&dir).copied()
    }

    pub fn new(dir_sprite_map: BTreeMap<MovementDirection, (u32, u32)>) -> Self {
        PlayerSprite { dir_sprite_map }
    }
}
