use bevy::prelude::{Parent, Query, Res};

use crate::{
    components::{
        collision::probed_rigid_body::ProbedRigidBody,
        rendering::{
            drawable::Sprite,
            sprites::{MovementDirection, PlayerSprite},
        },
    },
    resources::world::environment::EnvironmentConfig,
};

pub fn player_sprite_update(
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
