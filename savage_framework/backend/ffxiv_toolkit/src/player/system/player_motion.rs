use bevy::{
    math::Vec3,
    prelude::{Query, Res},
    time::Time,
};
use bevy_rapier3d::{control::KinematicCharacterController, plugin::RapierConfiguration};

use crate::player::{
    component::{PlayerController, PlayerTag},
    resource::input_actions::BaseInputActions,
};

pub fn player_motion_system(
    time: Res<Time>,
    rapier_config: Res<RapierConfiguration>,
    mut player_motion_query: Query<(
        &PlayerTag,
        &PlayerController,
        &mut KinematicCharacterController,
    )>,
) {
    for (_, controller, mut kinematic_controller) in player_motion_query.iter_mut() {
        let presed_actions = controller.input_pressed;

        let left: f32 = if presed_actions & BaseInputActions::Left as u64 != 0 { -1.0 } else { 0.0 };
        let right: f32 = if presed_actions & BaseInputActions::Right as u64 != 0 { 1.0 } else { 0.0 };
        let up: f32 = if presed_actions & BaseInputActions::Up as u64 != 0 { 1.0 } else { 0.0 };
        let down: f32 = if presed_actions & BaseInputActions::Down as u64 != 0 { -1.0 } else { 0.0 };

        // Gravity
        let pull = if !controller.grounded {
            (rapier_config.gravity * time.delta_seconds()) / 2.0 // Second time multiply will come in the vector as a whole.
        } else { Vec3::ZERO };

        let motion_vector = Vec3::new(left + right, 0.0, up + down).normalize() * controller.speed + pull;
        kinematic_controller.translation = Some(motion_vector * time.delta_seconds());
    }
}
