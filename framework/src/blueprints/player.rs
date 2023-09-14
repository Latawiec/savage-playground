use bevy::prelude::{Commands, Bundle, BuildChildren, Transform, GlobalTransform};
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::components::player::classes::Class;
use crate::components::player::identity::Identity;
use crate::components::player::jobs::*;
use crate::components::player::roles::Role;

use crate::components::collision::player::PlayerRigidBody;

#[derive(Bundle)]
struct PlayerRigidBodyBundle {
    rigid_body: PlayerRigidBody,
    collider: Collider,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl PlayerRigidBodyBundle {
    const PLAYER_COLLISION_RADIUS: f32 = 0.25;

    pub fn new() -> Self {
        PlayerRigidBodyBundle {
            rigid_body: PlayerRigidBody::default(),
            collider: Collider::ball(Self::PLAYER_COLLISION_RADIUS),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Bundle)]
struct PlayerHitboxBundle {
    sensor_tag: Sensor,
    collider: Collider,
}

impl PlayerHitboxBundle {
    const PLAYER_HITBOX_RADIUS: f32 = 0.5;

    pub fn new() -> Self {
        PlayerHitboxBundle {
            sensor_tag: Sensor::default(),
            collider: Collider::ball(Self::PLAYER_HITBOX_RADIUS),
        }
    }
}


pub fn spawn_player(
    mut commands: Commands,
    name: String,
    job_bundle: (Class, Role, Job)
) {
    let player_hitbox_entity = commands
        .spawn(PlayerHitboxBundle::new())
        .id();

    let player_rigid_body_entity = commands
        .spawn(Identity::new(name))
        .insert(job_bundle)
        .insert(PlayerRigidBodyBundle::new())
        .add_child(player_hitbox_entity)
        .id()
        ;
}
