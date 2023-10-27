use std::sync::mpsc::sync_channel;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use framework::game::blueprints::player_blueprint::Player;
use framework::game::common::player_type::new_player_id;
use framework::game::debug::local_input::LocalPlayerInput;
use framework::game::player::component::jobs::PALADIN;
use framework::game::player::component::raid_roles::RaidRole;
use framework::game::world::resource::EnvironmentConfig;
use framework::io::resource::PlayerInputManager;
use framework::plugin::FrameworkPlugin;
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

    let local_input_res = LocalPlayerInput{ player_id };
    let input_manager = PlayerInputManager::default();
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
