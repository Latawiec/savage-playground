use bevy::input::keyboard;
use bevy::{app::ScheduleRunnerPlugin, utils::Duration, prelude::*};

pub mod components;
pub mod resources;

use components::ball::{Ball, Direction};
use components::palette::{Palette, Side};
use resources::settings::Settings;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong".into(),
                        resolution: (1280., 960.).into(), 
                        resizable: false,
                        ..default()
                    }),
                ..default()
        }))
        .insert_resource(Settings{ resolution: [1280, 960] })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_palletes, move_ball))
        .run();
}

fn setup(mut commands: Commands, app_settings: Res<Settings>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(25.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-600., 0., 0.)),
        ..default()
    }, Palette{side: Side::Left, up_key: KeyCode::W, down_key: KeyCode::S}));

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.75, 0.25, 0.25),
            custom_size: Some(Vec2::new(25.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(600., 0., 0.)),
        ..default()
    }, Palette{side: Side::Right, up_key: KeyCode::Up, down_key: KeyCode::Down}));

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.75, 0.25),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }, Ball{direction: Direction::Left, speed: 5.}));
}

const MOVE_SPEED: f32 = 4.0;

fn move_palletes(mut query: Query<(&Palette, &mut Transform, &Sprite)>, keyboard_input: Res<Input<KeyCode>>, app_settings: Res<Settings>) {
    let top_edge = app_settings.resolution[1] as f32 / 2.;
    let bottom_edge = -(app_settings.resolution[1] as f32 / 2.);

    for (pal, mut transform, sprite) in query.iter_mut() {
        let half_height = sprite.custom_size.unwrap()[1] / 2.;

        if keyboard_input.pressed(pal.up_key) {
            if transform.translation.y + half_height < top_edge {
                transform.translation.y += MOVE_SPEED;
            }
        }
        else if keyboard_input.pressed(pal.down_key) {
            if transform.translation.y - half_height > bottom_edge {
                transform.translation.y -= MOVE_SPEED;
            }
        }
    }
}

fn move_ball(mut query: Query<(&mut Ball, &mut Transform, &Sprite)>, palletes: Query<(&Palette, &Transform, &Sprite)>, app_settings: Res<Settings>) {
    for (ball, transform, sprite) in query.iter_mut() {
        let half_width = sprite.custom_size.unwrap()[0] / 2.;
        let half_height = sprite.custom_size.unwrap()[1] / 2.;

        for (pal, pal_transform, pal_sprite) in palletes.iter() {
            let pal_half_width = pal_sprite.custom_size.unwrap()[0] / 2.;
            let pal_half_height = pal_sprite.custom_size.unwrap()[1] / 2.;
        if pal.side == Side::Left {
                if transform.translation.x - half_width < pal_transform.translation.x + pal_half_width {
                    if transform.translation.y + half_height < pal_transform + pal_half_height a
                }
            }
        }
    }
}