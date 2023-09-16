use std::collections::BTreeMap;

use bevy::prelude::{BuildChildren, Commands, Entity};
use bevy::transform::TransformBundle;

use crate::components::collision::hitbox::HitboxBundle;
use crate::components::collision::probed_rigid_body::{PhysicsProbeBundle, ProbedRigidBody};
use crate::components::player::classes::Class;
use crate::components::player::identity::Identity;
use crate::components::player::jobs::*;
use crate::components::player::raid_roles::RaidRole;
use crate::components::player::roles::Role;

use crate::components::rendering::drawable::Sprite;
use crate::components::rendering::sprites::{MovementDirection, PlayerSprite};
use crate::types::player::PlayerID;

pub struct Player();

impl Player {
    const PLAYER_COLLISION_RADIUS: f32 = 12.25;
    const PLAYER_HITBOX_RADIUS: f32 = 25.5;

    pub fn spawn(
        commands: &mut Commands,
        id: PlayerID,
        name: String,
        job_bundle: (Class, Role, Job),
        role: RaidRole,
    ) -> Entity {
        let probe = commands
            .spawn(PhysicsProbeBundle::new(Self::PLAYER_COLLISION_RADIUS))
            .id();
        let hitbox = commands
            .spawn(HitboxBundle::new(Self::PLAYER_HITBOX_RADIUS))
            .id();
        let drawing: Entity = commands
            .spawn(Sprite::default())
            .insert(PlayerSprite::new(BTreeMap::from([
                (MovementDirection::Forward, (0, 0)),
                (MovementDirection::Back, (0, 1)),
                (MovementDirection::Left, (1, 0)),
                (MovementDirection::Right, (1, 1)),
            ])))
            .id();

        commands
            .spawn(ProbedRigidBody::default())
            .insert(TransformBundle::default())
            .insert(Identity::new(name, id))
            .insert(job_bundle)
            .insert(role)
            .add_child(probe)
            .add_child(hitbox)
            .add_child(drawing)
            .id()
    }
}
