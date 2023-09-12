use bevy::{prelude::*, time::TimePlugin, ecs::schedule::AnonymousSet};
use bevy_rapier2d::{control, na::Quaternion, prelude::*};
use savage_playgrounds::core::{components::interaction::{self, player::{player_rigid_body_system, PlayerRigidBody}}, defs::player};

mod savage_playgrounds;

#[derive(Component)]
struct Controller {
    direction: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_systems(FixedUpdate, player_rigid_body_system)
        .add_system(print_ball_altitude)
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

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    let mut ground_transform = Transform::from_xyz(0.0, -100.0, 0.0);
    ground_transform.rotate_z(std::f32::consts::FRAC_PI_6 / 10.0);

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(ground_transform));

    /* Create the bouncing ball. */
    let mut test_transform = Transform::from_xyz(0.0, 400.0, 0.0);
    test_transform.rotate_z(-std::f32::consts::PI);

    commands
        .spawn(Collider::compound(vec![
            (Vec2::ZERO, Real::default(), Collider::cuboid(20.0, 20.0)),
            (Vec2::new(0.0, 30.0), Real::default(), Collider::ball(50.0)),
        ]))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(test_transform))
        .insert(Controller {
            direction: Vec2::new(10.0, -40.0),
        })
        .insert(PlayerRigidBody {
            velocity: Vec2::new(10.0, -40.0),
        });

    // Cone
    //commands.spawn(indicators::aoe::AreaOfEffectBundle::cone(std::f32::consts::FRAC_PI_2, 150.0));

    // Cleave
    //commands.spawn(indicators::aoe::AreaOfEffectBundle::cleave(std::f32::consts::PI * 1.5, 150.0, 20));

    // Donut
    // commands.spawn(interaction::aoe::AreaOfEffectBundle::donut(50.0, 150.0, 30));

    // Beam
    commands
        .spawn(interaction::aoe::AreaOfEffectBundle::beam(50.0, 500.0))
        .insert(player::roles::Role::Tank)
        .insert(player::jobs::PALADIN);
}

fn print_ball_altitude(positions: Query<&Transform, With<Controller>>) {
    for transform in positions.iter() {
        // println!("Ball altitude: {}", transform.translation.y);
    }
}

