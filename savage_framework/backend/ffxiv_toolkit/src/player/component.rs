use bevy::prelude::{Bundle, Component, SpatialBundle};
use bevy_rapier3d::{control::KinematicCharacterController, dynamics::RigidBody};

#[derive(Component)]
pub struct PlayerTag;

type InputActionsStorageType = u64;
#[derive(Component)]
pub struct PlayerController {
    pub input_just_pressed: InputActionsStorageType,
    pub input_just_released: InputActionsStorageType,
    pub input_pressed: InputActionsStorageType,
    pub speed: f32,
    pub grounded: bool,
}

#[derive(Component)]
pub struct PlayerInfo {
    pub player_id: u64,
    pub player_name: Option<String>,
    pub active: bool,
}

#[derive(Bundle)]
pub struct FfxivToolkitPlayerBundle {
    tag: PlayerTag,
    controller: PlayerController,
    info: PlayerInfo,
    spatial: SpatialBundle,
    rapier_controller: KinematicCharacterController,
    rapier_rigid_body: RigidBody,
}
