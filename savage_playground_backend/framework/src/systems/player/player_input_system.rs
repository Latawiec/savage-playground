use bevy::prelude::{Query, Res, Vec2};

use crate::{
    components::{collision::probed_rigid_body::ProbedRigidBody, player::identity::Identity},
    resources::{
        world::environment::EnvironmentConfig,
    }, io::{resource::PlayerInputManager, default_key_flags::DefaultKeyFlags},
};

pub fn player_motion_input_system(
    environment: Res<EnvironmentConfig>,
    mut input_manager: Res<PlayerInputManager>,
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
