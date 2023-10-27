use std::collections::BTreeMap;

use bevy::prelude::{BuildChildren, Commands, Entity};
use bevy::transform::TransformBundle;

use crate::game::collision::component::hitbox::HitboxBundle;
use crate::game::collision::component::probed_rigid_body::{PhysicsProbeBundle, ProbedRigidBody};
use crate::game::collision::component::tags::PlayerHitboxTag;
use crate::game::common::player_type::PlayerID;
use crate::game::player::component::classes::Class;
use crate::game::player::component::identity::Identity;
use crate::game::player::component::jobs::Job;
use crate::game::player::component::raid_roles::RaidRole;
use crate::game::player::component::roles::Role;
use crate::game::rendering::component::drawable::Sprite;
use crate::game::rendering::component::sprites::{MovementDirection, PlayerSprite};

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
            .insert(PlayerHitboxTag)
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
