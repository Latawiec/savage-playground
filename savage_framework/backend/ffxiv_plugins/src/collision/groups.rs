pub type CollisionGroup = u32;

pub const COLLISION_PLAYER: u32 = 1u32 << 0;
pub const COLLISION_OBSTACLES: u32 = 1u32 << 1;

pub const SENSOR_PLAYER_HITBOX: u32 = 1u32 << 2;
pub const SENSOR_AOE_HITBOX: u32 = 1u32 << 3;
pub const SENSOR_ENEMY_HITBOX: u32 = 1u32 << 4;