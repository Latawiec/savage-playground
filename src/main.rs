// use bevy::{prelude::*, time::TimePlugin, ecs::schedule::AnonymousSet};
// use bevy_rapier2d::{control, na::Quaternion, prelude::*};
// use savage_playgrounds::core::{components::interaction::{self, player::{player_rigid_body_system, PlayerRigidBody}}, defs::player};

// mod savage_playgrounds;

// #[derive(Component)]
// struct Controller {
//     direction: Vec2,
// }

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
//         .add_plugin(RapierDebugRenderPlugin::default())
//         .add_startup_system(setup_graphics)
//         .add_startup_system(setup_physics)
//         .add_systems(FixedUpdate, player_rigid_body_system)
//         .add_system(print_ball_altitude)
//         // .add_systems(Update, check_path)
//         .insert_resource(RapierConfiguration {
//             gravity: Vec2::ZERO,
//             ..Default::default()
//         })
//         .run();
// }

// fn setup_graphics(mut commands: Commands) {
//     // Add a camera so we can see the debug-render.
//     commands.spawn(Camera2dBundle::default());
// }

// fn setup_physics(mut commands: Commands) {
//     /* Create the ground. */
//     let mut ground_transform = Transform::from_xyz(0.0, -100.0, 0.0);
//     ground_transform.rotate_z(std::f32::consts::FRAC_PI_6 / 10.0);

//     commands
//         .spawn(RigidBody::Fixed)
//         .insert(Collider::cuboid(500.0, 50.0))
//         .insert(TransformBundle::from(ground_transform));

//     /* Create the bouncing ball. */
//     let mut test_transform = Transform::from_xyz(0.0, 400.0, 0.0);
//     test_transform.rotate_z(-std::f32::consts::PI);

//     commands
//         .spawn(Collider::compound(vec![
//             (Vec2::ZERO, Real::default(), Collider::cuboid(20.0, 20.0)),
//             (Vec2::new(0.0, 30.0), Real::default(), Collider::ball(50.0)),
//         ]))
//         .insert(Restitution::coefficient(0.7))
//         .insert(TransformBundle::from(test_transform))
//         .insert(Controller {
//             direction: Vec2::new(10.0, -40.0),
//         })
//         .insert(PlayerRigidBody {
//             velocity: Vec2::new(10.0, -40.0),
//         });

//     // Cone
//     //commands.spawn(indicators::aoe::AreaOfEffectBundle::cone(std::f32::consts::FRAC_PI_2, 150.0));

//     // Cleave
//     //commands.spawn(indicators::aoe::AreaOfEffectBundle::cleave(std::f32::consts::PI * 1.5, 150.0, 20));

//     // Donut
//     // commands.spawn(interaction::aoe::AreaOfEffectBundle::donut(50.0, 150.0, 30));

//     // Beam
//     commands
//         .spawn(interaction::aoe::AreaOfEffectBundle::beam(50.0, 500.0))
//         .insert(player::roles::Role::Tank)
//         .insert(player::jobs::PALADIN);
// }

// fn print_ball_altitude(positions: Query<&Transform, With<Controller>>) {
//     for transform in positions.iter() {
//         // println!("Ball altitude: {}", transform.translation.y);
//     }
// }

use std::sync::mpsc::{channel, sync_channel};

use bevy::{prelude::*, time::TimePlugin, ecs::schedule::AnonymousSet};
use bevy_rapier2d::{control, na::Quaternion, prelude::*};

use framework::{*, resources::{io::input::{NewInput, InputManager}, world::environment::{self, EnvironmentConfig}}, debug::local_input::{LocalInput, self, local_input_system}, types::player::new_player_id, blueprints::player::spawn_player, components::player::{jobs::PALADIN, raid_roles::RaidRole}};

fn main() {



    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(0.01))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(create_test_stuff)
        .add_systems(Update, framework::systems::player::player_rigid_body_system::player_rigid_body_system)
        .add_systems(PreUpdate, framework::systems::player::player_input_system::player_input_system)
        .add_systems(PostUpdate, local_input_system)
        .add_systems(Update, spin_wall)
        // .add_systems(Update, check_path)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct TestWall;

fn create_test_stuff(mut commands: Commands) {

    let player_id = new_player_id();
    let (tx, rx) = sync_channel::<NewInput>(10);

    let local_input_res = LocalInput::new(player_id, tx);
    let input_manager = InputManager::new(rx);
    let environment = EnvironmentConfig {
        north: Vec2::new(0.0, 1.0),
        east: Vec2::new(1.0, 0.0),
        environment_scale: 1.0,
        movement_speed: 200.0,
    };

    commands.insert_resource(environment);
    commands.insert_resource(input_manager);
    commands.insert_resource(local_input_res);
    spawn_player(&mut commands, player_id, "Henry".to_string(), PALADIN, RaidRole::MT);

    let mut ground_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    commands.spawn(RigidBody::Fixed)
    .insert(Collider::cuboid(100.0, 50.0))
    .insert(TransformBundle::from(ground_transform))
    .insert(TestWall);
}

fn spin_wall(
    mut wall: Query<(&TestWall, &mut Transform)>,
) {
    for (_, mut transform) in wall.iter_mut() {
        transform.rotate_local_z(std::f32::consts::PI / 120.0);
    }
}