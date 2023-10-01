use bevy::{prelude::{Bundle, GlobalTransform, Transform, Component}, transform::TransformBundle};
use bevy_rapier2d::prelude::{Collider, Sensor, CollisionGroups, Group, RigidBody};

use super::collision_groups::*;


#[derive(Bundle)]
pub struct HitboxBundle {
    sensor_tag: Sensor,
    collider: Collider,
    rigid_body: RigidBody,
    collision_groups: CollisionGroups,
    transform: TransformBundle,
}

impl HitboxBundle {
    pub fn new(radius: f32) -> Self {
        HitboxBundle {
            sensor_tag: Sensor::default(),
            collider: Collider::ball(radius),
            rigid_body: RigidBody::Dynamic, // Intersection test doesn't work otherwise.
            collision_groups: Self::collision_groups(),
            transform: TransformBundle::default()
        }
    }

    fn collision_groups() -> CollisionGroups {
        CollisionGroups {
            memberships: Group::from(Group::from_bits_truncate(SENSOR_PLAYER_HITBOX)),
            filters: Group::from(Group::from_bits_truncate(SENSOR_AOE_HITBOX)),
        }
    }
}
