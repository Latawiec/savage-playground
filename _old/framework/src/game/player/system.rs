use bevy::prelude::{Parent, Query, Res, Vec2};

use crate::game::{
    collision::component::probed_rigid_body::ProbedRigidBody,
    io::{default_key_flags::DefaultKeyFlags, resource::PlayerInputManager},
    rendering::component::{
        drawable::Sprite,
        sprites::{MovementDirection, PlayerSprite},
    },
    world::resource::EnvironmentConfig,
};

use super::component::identity::Identity;

pub fn player_motion_input_system(
    environment: Res<EnvironmentConfig>,
    input_manager: Res<PlayerInputManager>,
    mut players: Query<(&mut ProbedRigidBody, &Identity)>,
) {
    // Apply inputs.
    for (mut rigid_body, identity) in players.iter_mut() {
        let player_id = &identity.id;

        if let Some(input) = input_manager.get_input_state(&player_id) {
            if input.changed() {
                // Motion.
                let mut new_direction = Vec2::new(0.0, 0.0);
                if input.is_key_down(DefaultKeyFlags::Up.into()) {
                    new_direction += environment.forward;
                }
                if input.is_key_down(DefaultKeyFlags::Down.into()) {
                    new_direction -= environment.forward;
                }
                if input.is_key_down(DefaultKeyFlags::Right.into()) {
                    new_direction += environment.right;
                }
                if input.is_key_down(DefaultKeyFlags::Left.into()) {
                    new_direction -= environment.right;
                }

                rigid_body.velocity =
                    new_direction.normalize_or_zero() * environment.movement_speed;
                // Buttons ?...
                // TODO: More if needed
            }
        }
    }
}

pub fn player_sprite_update_system(
    environment: Res<EnvironmentConfig>,
    mut player_sprites: Query<(&Parent, &PlayerSprite, &mut Sprite)>,
    player_rigid_body: Query<&ProbedRigidBody>,
) {
    for (parent, player_sprite, mut sprite) in player_sprites.iter_mut() {
        match player_rigid_body.get(parent.get()) {
            Err(e) => {
                tracing::error!("Couldn't get player RigidBody for Sprite update. {}", e);
                return;
            }
            Ok(rigid_body) => {
                let dir = rigid_body.velocity.normalize();
                if dir.is_nan() {
                    // Don't change anything. We're standing still.
                    continue;
                }

                let forward = environment.forward.normalize_or_zero();
                let right = environment.right.normalize_or_zero();

                let (_, direction) = vec![
                    (dir.dot(-right), MovementDirection::Left),
                    (dir.dot(right), MovementDirection::Right),
                    (dir.dot(-forward), MovementDirection::Back),
                    (dir.dot(forward), MovementDirection::Forward),
                ]
                .iter()
                .max_by(|(lhs, _), (rhs, _)| lhs.total_cmp(rhs))
                .unwrap()
                .to_owned();

                if let Some(new_selection) = player_sprite.get_selection(direction) {
                    sprite.active_tile = new_selection;
                } else {
                    tracing::error!("Missing mapping for direction: {:?}", direction);
                }
            }
        }
    }
}
