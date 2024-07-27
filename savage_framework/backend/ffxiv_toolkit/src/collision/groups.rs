use bevy_rapier3d::geometry::Group;

pub type CollisionGroup = u32;

pub const COLLISION_PLAYER: Group = Group::GROUP_1;
pub const COLLISION_OBSTACLES: Group = Group::GROUP_2;

pub const SENSOR_PLAYER_HITBOX: Group = Group::GROUP_3;
pub const SENSOR_AOE_HITBOX: Group = Group::GROUP_4;
pub const SENSOR_ENEMY_HITBOX: Group = Group::GROUP_5;


pub const SENSOR_UNREACHABLE: Group = Group::GROUP_32;