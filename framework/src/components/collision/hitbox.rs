use bevy::prelude::{Bundle, GlobalTransform, Transform, Component};
use bevy_rapier2d::prelude::{Collider, Sensor};

#[derive(Component, Default)]
pub struct PlayerHitboxTag;

#[derive(Bundle)]
pub struct HitboxBundle {
    sensor_tag: Sensor,
    collider: Collider,
    transform: Transform,
    global_transform: GlobalTransform,
    tag: PlayerHitboxTag,
}

impl HitboxBundle {
    pub fn new(radius: f32) -> Self {
        HitboxBundle {
            sensor_tag: Sensor::default(),
            collider: Collider::ball(radius),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            tag: PlayerHitboxTag::default(),
        }
    }
}
