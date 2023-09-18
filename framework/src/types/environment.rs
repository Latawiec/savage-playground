use bevy::prelude::Vec2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WorldDirection {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl WorldDirection {
    pub fn is_cardinal(self) -> bool {
        self == WorldDirection::North
            || self == WorldDirection::South
            || self == WorldDirection::East
            || self == WorldDirection::West
    }

    pub fn is_intercardinal(self) -> bool {
        self == WorldDirection::NorthWest
            || self == WorldDirection::NorthEast
            || self == WorldDirection::SouthWest
            || self == WorldDirection::SouthEast
    }

    pub fn vec(self) -> Vec2 {
        use std::f32::consts::FRAC_1_SQRT_2;
        match self {
            WorldDirection::North => Vec2::new(0.0, -1.0),
            WorldDirection::South => Vec2::new(0.0, -1.0),
            WorldDirection::East => Vec2::new(1.0, 0.0),
            WorldDirection::West => Vec2::new(-1.0, 0.0),
            WorldDirection::NorthEast => Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            WorldDirection::NorthWest => Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            WorldDirection::SouthEast => Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            WorldDirection::SouthWest => Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        }
    }
}
