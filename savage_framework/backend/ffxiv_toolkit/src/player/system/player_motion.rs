use bevy::{
    math::Vec3,
    prelude::{Query, Res, With},
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
        &mut PlayerController,
        &mut KinematicCharacterController,
    ), With<PlayerTag>>,
) {
    for (mut controller, mut kinematic_controller) in player_motion_query.iter_mut() {
        let presed_actions = controller.input_pressed;

        let motion_vector = if presed_actions == 0 {
            Vec3::ZERO
        } else {

            let left: f32 = if presed_actions & BaseInputActions::Left as u64 != 0 { -1.0 } else { 0.0 };
            let right: f32 = if presed_actions & BaseInputActions::Right as u64 != 0 { 1.0 } else { 0.0 };
            let up: f32 = if presed_actions & BaseInputActions::Up as u64 != 0 { 1.0 } else { 0.0 };
            let down: f32 = if presed_actions & BaseInputActions::Down as u64 != 0 { -1.0 } else { 0.0 };

            Vec3::new(left + right, 0.0, up + down).normalize() * controller.speed
        };
        // Gravity
        controller.fall_velocity += rapier_config.gravity * time.delta_seconds();
        controller.fall_velocity = controller.fall_velocity.clamp(Vec3::new(-10.0, -10.0, -10.0), Vec3::new(10.0, 10.0, 10.0));
        if presed_actions & BaseInputActions::Jump as u64 != 0 {
            controller.fall_velocity = Vec3::new(0.0, 8.0, 0.0);
        }
        
        let pull = if !controller.grounded {
            controller.fall_velocity
        } else { Vec3::ZERO };

        let velocity_vec = motion_vector + pull;
        kinematic_controller.translation = Some(velocity_vec * time.delta_seconds());
    }
}
