use bevy::prelude::{Vec2, World};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    pub const CARDINALS: [WorldDirection; 4] = [
        Self::North, 
        Self::East,
        Self::South,
        Self::West,
    ];

    pub const INTERCARDINALS: [WorldDirection; 4] = [
        Self::NorthEast, 
        Self::SouthEast,
        Self::SouthWest,
        Self::NorthWest,
    ];

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

    pub fn opposite(&self) -> WorldDirection {
        match self {
            WorldDirection::North => WorldDirection::South,
            WorldDirection::South => WorldDirection::North,
            WorldDirection::East => WorldDirection::West,
            WorldDirection::West => WorldDirection::East,
            WorldDirection::NorthWest => WorldDirection::SouthEast,
            WorldDirection::NorthEast => WorldDirection::SouthWest,
            WorldDirection::SouthWest => WorldDirection::NorthEast,
            WorldDirection::SouthEast => WorldDirection::NorthWest,
        }
    }

    pub fn perpendicular_clockwise(&self) -> WorldDirection {
        match self {
            WorldDirection::North => WorldDirection::East,
            WorldDirection::East => WorldDirection::South,
            WorldDirection::South => WorldDirection::West,
            WorldDirection::West => WorldDirection::North,
            WorldDirection::NorthEast => WorldDirection::SouthEast,
            WorldDirection::SouthEast => WorldDirection::SouthWest,
            WorldDirection::SouthWest => WorldDirection::NorthWest,
            WorldDirection::NorthWest => WorldDirection::NorthEast,
        }
    }

    pub fn is_opposite(&self, other: &WorldDirection) -> bool {
        *self == other.opposite()
    }

    pub fn vec(self) -> Vec2 {
        use std::f32::consts::FRAC_1_SQRT_2;
        match self {
            WorldDirection::North => Vec2::new(0.0, 1.0),
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
