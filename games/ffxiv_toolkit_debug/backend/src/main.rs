use std::{thread::sleep, time::Duration};

use bevy::{app::{App, PanicHandlerPlugin, PluginGroup, Startup, Update}, core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin}, diagnostic::DiagnosticsPlugin, input::InputPlugin, log::LogPlugin, math::{Quat, Vec3}, prelude::{Camera3dBundle, Commands, EventReader, HierarchyPlugin, Query, Transform, TransformBundle, TransformPlugin, With}, scene::ScenePlugin, window::WindowPlugin, DefaultPlugins};
use bevy_rapier3d::{prelude::{Collider, Restitution, RigidBody}, render::{ColliderDebugColor, RapierDebugRenderPlugin}};
use ffxiv_toolkit::{input::event::FFXIVGameInputEvent, player::component::{FfxivToolkitPlayerBundle, PlayerController, PlayerInfo}, plugin::FfxivToolkitPlugin, settings::event::PlayerJoinedEvent};
use game_config::game_args::GameArgs;
use tracing::info;

fn main() {
    let game_args = GameArgs::new("FFXIV Toolkit Debug - debugging Bevy-based game.", "FFXIV Toolkit Debug", tracing::Level::DEBUG, "wgpu=warn,naga=warn");
    if let Err(err) = game_args.process() {
        eprintln!("Error: {:?}", err);
        return;
    }

    App::new()
        .add_plugins(DefaultPlugins.build()
            .disable::<bevy::log::LogPlugin>())
        .add_plugins(FfxivToolkitPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())

        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, setup_player)
        .add_systems(Update, print_ball_altitude)
        
        .run();
}

fn setup_player(
    mut ev_player_joined: EventReader<PlayerJoinedEvent>,
    mut commands: Commands
) {
    for event in ev_player_joined.read() {
        let player_id = event.player_id;
        commands
            .spawn(FfxivToolkitPlayerBundle {
                tag: ffxiv_toolkit::player::component::PlayerTag,
                controller: PlayerController {
                    input_just_pressed: 0,
                    input_just_released: 0,
                    input_pressed: 0,
                    fall_velocity: Vec3::ZERO,
                    speed: 10.0,
                    grounded: false,
                },
                info: PlayerInfo {
                    player_id,
                    player_name: Some(format!("Player {}", player_id)),
                    active: true,
                },
                spatial: Default::default(),
                rapier_controller: Default::default(),
                rapier_rigid_body: Default::default(),
            })
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::ball(0.25))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 3.0, 0.0)));
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    let bundle = Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    commands.spawn(bundle);
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(2.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(1.0, 1.0, 1.0)));
}

fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut transform in positions.iter_mut() {
        // dbg!(transform.rotation.to_axis_angle());
        transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
        //println!("Ball altitude: {}", transform.translation.y);
    }
}