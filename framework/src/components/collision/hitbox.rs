use bevy::{prelude::{Bundle, GlobalTransform, Transform, Component}, transform::TransformBundle};
use bevy_rapier2d::prelude::{Collider, Sensor, CollisionGroups, Group};

use super::collision_groups::*;

#[derive(Component, Default)]
pub struct PlayerHitboxTag;

#[derive(Bundle)]
pub struct HitboxBundle {
    sensor_tag: Sensor,
    collider: Collider,
    collision_groups: CollisionGroups,
    transform: TransformBundle,
    tag: PlayerHitboxTag,
}

impl HitboxBundle {
    pub fn new(radius: f32) -> Self {
        HitboxBundle {
            
            sensor_tag: Sensor::default(),
            collider: Collider::ball(radius),
            collision_groups: Self::collision_groups(),
            transform: TransformBundle::default(),
            tag: PlayerHitboxTag::default(),
        }
    }

    fn collision_groups() -> CollisionGroups {
        CollisionGroups {
            memberships: Group::from(Group::from_bits_truncate(SENSOR_PLAYER_HITBOX)),
            filters: Group::from(Group::from_bits_truncate(SENSOR_AOE_HITBOX)),
        }
    }
}
