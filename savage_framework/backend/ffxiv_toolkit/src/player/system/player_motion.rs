use bevy::prelude::Query;
use bevy_rapier3d::control::KinematicCharacterController;

use crate::player::component::{PlayerController, PlayerTag};



pub fn player_motion_system(
    mut player_motion_query: Query<(&PlayerTag, &PlayerController, &mut KinematicCharacterController)>
) {
    for (_, controller, kinematic_controller) in player_motion_query.iter_mut() {
        let mu 
    }
}