use bevy::prelude::{Query, Res, ResMut, Vec2};

use crate::{
    components::{collision::probed_rigid_body::ProbedRigidBody, player::identity::Identity},
    resources::{
        io::input::{InputManager, KeyFlag},
        world::environment::EnvironmentConfig,
    },
};

pub fn player_input_system(
    environment: Res<EnvironmentConfig>,
    mut input_manager: ResMut<InputManager>,
    mut players: Query<(&mut ProbedRigidBody, &Identity)>,
) {
    // Process new input first.
    if !input_manager.process_input() {
        tracing::error!("Couldn't process input.");
        return;
    }

    // Apply inputs.
    for (mut rigid_body, identity) in players.iter_mut() {
        let player_id = &identity.id;

        if let Some(input) = input_manager.players_input.get(player_id) {
            if input.changed() {
                // Motion.
                let mut new_direction = Vec2::new(0.0, 0.0);
                if input.is_key_down(KeyFlag::Up) {
                    new_direction += environment.forward;
                }
                if input.is_key_down(KeyFlag::Down) {
                    new_direction -= environment.forward;
                }
                if input.is_key_down(KeyFlag::Right) {
                    new_direction += environment.right;
                }
                if input.is_key_down(KeyFlag::Left) {
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
