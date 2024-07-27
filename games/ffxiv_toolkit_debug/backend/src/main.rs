use bevy::{app::{App, PanicHandlerPlugin, Startup, Update}, core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin}, diagnostic::DiagnosticsPlugin, input::InputPlugin, math::{Quat, Vec3}, prelude::{Camera3dBundle, Commands, HierarchyPlugin, Query, Transform, TransformBundle, TransformPlugin, With}, scene::ScenePlugin, window::WindowPlugin, DefaultPlugins};
use bevy_rapier3d::{prelude::{Collider, Restitution, RigidBody}, render::RapierDebugRenderPlugin};
use ffxiv_toolkit::plugin::FfxivToolkitPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FfxivToolkitPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())

        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        
        .run();
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
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_ball_altitude(mut positions: Query<&mut Transform, With<RigidBody>>) {
    for mut transform in positions.iter_mut() {
        // dbg!(transform.rotation.to_axis_angle());
        transform.rotation = Quat::from_rotation_z(270_f32.to_radians());
        //println!("Ball altitude: {}", transform.translation.y);
    }
}