use bevy::{math::Vec3, prelude::{Bundle, Component, SpatialBundle}};
use bevy_rapier3d::{control::KinematicCharacterController, dynamics::RigidBody};

#[derive(Component, Default)]
pub struct PlayerTag;

type InputActionsStorageType = u64;
#[derive(Component, Default)]
pub struct PlayerController {
    pub input_just_pressed: InputActionsStorageType,
    pub input_just_released: InputActionsStorageType,
    pub input_pressed: InputActionsStorageType,
    pub speed: f32,
    pub fall_velocity: Vec3,
    pub grounded: bool,
}

#[derive(Component, Default)]
pub struct PlayerInfo {
    pub player_id: u64,
    pub player_name: Option<String>,
    pub active: bool,
}

#[derive(Bundle, Default)]
pub struct FfxivToolkitPlayerBundle {
    pub tag: PlayerTag,
    pub controller: PlayerController,
    pub info: PlayerInfo,
    pub spatial: SpatialBundle,
    pub rapier_controller: KinematicCharacterController,
    pub rapier_rigid_body: RigidBody,
}
