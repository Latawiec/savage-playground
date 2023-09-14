use bevy::ecs::component::Component;
use bevy::prelude::{Entity, GlobalTransform, Query, Res, Transform, Vec2, Vec3};
use bevy::time::fixed_timestep::FixedTime;
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, TOIStatus, Vect};

/// Player Rigid Body works similarly to bevy RigidBody::KinematicVelocityBased.
/// The only difference is that [PlayerRigidBody] reacts to environment.
/// If a solid wall moves towards [PlayerRigidBody], [PlayerRigidBody] will be pushed without any impact on the wall.
#[derive(Component, Default)]
pub struct PlayerRigidBody {
    pub velocity: Vect,
}
