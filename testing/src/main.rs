use std::sync::mpsc::sync_channel;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use framework::plugin::FrameworkPlugin;
use framework::{
    blueprints::player::Player,
    components::player::{jobs::PALADIN, raid_roles::RaidRole},
    debug::local_input::LocalInput,
    resources::{
        io::input::{InputManager, NewInput},
        world::environment::EnvironmentConfig,
    },
    types::player::new_player_id,
};

use worlds::mechanics::ruby_glow::{RubyGlowOne, RubyGlowPlugin};
use worlds::mechanics::towers::TowersMechanicPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
        // this is just to test unlimited FPS.
            // .set(WindowPlugin {
            //     primary_window: Some(Window { present_mode: bevy::window::PresentMode::Immediate, ..Default::default() }),
            //     ..Default::default()})
        )
        .add_plugins(FrameworkPlugin)
        .add_plugins(TowersMechanicPlugin::default())
        .add_plugins(RubyGlowPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, create_test_stuff)
        .add_systems(Update, spin_wall)
        .run();
}

fn setup(mut commands: Commands) {
    // TowerSet::spawn(&mut commands, None);
    RubyGlowOne::spawn(&mut commands, None);
}

#[derive(Component)]
struct TestWall;

fn create_test_stuff(mut commands: Commands) {
    let player_id = new_player_id();
    let (tx, rx) = sync_channel::<NewInput>(10);

    let local_input_res = LocalInput::new(player_id, tx);
    let input_manager = InputManager::new(rx);
    let environment = EnvironmentConfig {
        forward: Vec2::new(0.0, 1.0),
        right: Vec2::new(1.0, 0.0),
        environment_scale: 1.0,
        movement_speed: 200.0,
    };

    commands.insert_resource(environment);
    commands.insert_resource(input_manager);
    commands.insert_resource(local_input_res);
    Player::spawn(
        &mut commands,
        player_id,
        "Henry".to_string(),
        PALADIN,
        RaidRole::MT,
    );

    let ground_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 50.0))
        .insert(TransformBundle::from(ground_transform))
        .insert(TestWall);

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(TransformBundle::from(ground_transform));
}

fn spin_wall(mut wall: Query<(&TestWall, &mut Transform)>) {
    for (_, mut transform) in wall.iter_mut() {
        transform.rotate_local_z(std::f32::consts::PI / 120.0);
    }
}
