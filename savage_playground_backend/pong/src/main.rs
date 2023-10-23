use bevy::input::keyboard;
use bevy::window::PresentMode;
use bevy::{app::ScheduleRunnerPlugin, utils::Duration, prelude::*};

pub mod components;
pub mod resources;

use components::ball::{Ball};
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
        .add_systems(FixedUpdate, (move_palletes, move_ball))
        .insert_resource(FixedTime::new_from_secs(1./60.))
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
    }, Ball{velocity: [1., 0.], speed: 5.}));
}

const MOVE_SPEED: f32 = 8.0;

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

fn move_ball(mut query: Query<(&mut Ball, &mut Transform, &Sprite), Without<Palette>>, palletes: Query<(&Palette, &Transform, &Sprite), Without<Ball>>, app_settings: Res<Settings>) {
    let top_edge = app_settings.resolution[1] as f32 / 2.;
    let bottom_edge = -(app_settings.resolution[1] as f32 / 2.);
    let left_edge = -(app_settings.resolution[0] as f32 / 2.);
    let right_edge = app_settings.resolution[0] as f32 / 2.;

    for (mut ball, mut transform, sprite) in query.iter_mut() {
        let half_width = sprite.custom_size.unwrap()[0] / 2.;
        let half_height = sprite.custom_size.unwrap()[1] / 2.;

        let ball_l = transform.translation.x - half_width;
        let ball_r = transform.translation.x + half_width;
        let ball_t = transform.translation.y + half_height;
        let ball_b = transform.translation.y - half_height;

        for (_, pal_transform, pal_sprite) in palletes.iter() {
            let pal_half_width = pal_sprite.custom_size.unwrap()[0] / 2.;
            let pal_half_height = pal_sprite.custom_size.unwrap()[1] / 2.;

            let pal_l = pal_transform.translation.x - pal_half_width;
            let pal_r = pal_transform.translation.x + pal_half_width;
            let pal_t = pal_transform.translation.y + pal_half_height;
            let pal_b = pal_transform.translation.y - pal_half_height;

            if pal_l <= ball_r && pal_r >= ball_l && pal_t >= ball_b && pal_b <= ball_t {
                let x = transform.translation.x - pal_transform.translation.x;
                let y = transform.translation.y - pal_transform.translation.y;

                let len = (x*x + y*y).sqrt();
                let x = x/len;
                let y = y/len;

                ball.velocity = [x,y];
                ball.speed *= 1.1;
            }
        }

        if transform.translation.y + half_height > top_edge || transform.translation.y - half_height < bottom_edge {
            ball.velocity[1] = -ball.velocity[1];
        }

        if transform.translation.x < left_edge || transform.translation.x > right_edge {
            transform.translation.x = 0.;
            transform.translation.y = 0.;
            ball.velocity[0] = -ball.velocity[0];
            ball.speed = 5.;
        }

        transform.translation.x += ball.velocity[0] * ball.speed;
        transform.translation.y += ball.velocity[1] * ball.speed;
    }
}