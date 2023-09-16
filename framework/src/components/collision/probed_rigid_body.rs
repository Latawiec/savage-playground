use bevy::prelude::{Bundle, Component, Entity, GlobalTransform, Transform};
use bevy_rapier2d::prelude::{Collider, RigidBody, Vect, Damping, LockedAxes};

#[derive(Component, Default)]
pub struct PhysicsProbe;

#[derive(Bundle)]
pub struct PhysicsProbeBundle {
    rigid_body: RigidBody,
    collider: Collider,
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
            transform: Transform::default(),
            global_transform: GlobalTransform::default(), 
            tag: PhysicsProbe::default(),
            lock: LockedAxes::ROTATION_LOCKED,
            damping: Damping { linear_damping: 50.0, angular_damping: 0.0 }
        }
    }
}

#[derive(Component)]
pub struct ProbedRigidBody {
    pub velocity: Vect,
    pub probe: Entity,
}

impl ProbedRigidBody {
    pub fn new(probe: Entity) -> Self {
        ProbedRigidBody {
            velocity: Default::default(),
            probe,
        }
    }
}
