use bevy::prelude::{Bundle, Component, GlobalTransform, Transform};
use bevy_rapier2d::prelude::{Collider, Damping, LockedAxes, RigidBody, Vect, CollisionGroups, Group};

use super::collision_groups::*;

#[derive(Component, Default)]
pub struct PhysicsProbe;

#[derive(Bundle)]
pub struct PhysicsProbeBundle {
    rigid_body: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups,
    transform: Transform,
    global_transform: GlobalTransform,
    tag: PhysicsProbe,
    lock: LockedAxes,
    damping: Damping,
}

impl PhysicsProbeBundle {
    pub fn new(radius: f32) -> Self {
        PhysicsProbeBundle {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(radius),
            collision_groups: Self::collision_groups(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            tag: PhysicsProbe::default(),
            lock: LockedAxes::ROTATION_LOCKED,
            damping: Damping {
                linear_damping: 50.0,
                angular_damping: 0.0,
            },
        }
    }

    fn collision_groups() -> CollisionGroups {
        CollisionGroups {
            memberships: Group::from(Group::from_bits_truncate(COLLISION_PLAYER)),
            filters: Group::from(Group::from_bits_truncate(COLLISION_PLAYER | COLLISION_OBSTACLES)),
        }
    }
}

#[derive(Component, Default)]
pub struct ProbedRigidBody {
    pub velocity: Vect,
}
