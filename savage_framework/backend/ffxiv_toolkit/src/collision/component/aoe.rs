use bevy::{prelude::{Bundle, Component}, transform::bundles::TransformBundle};
use bevy_rapier2d::geometry::{Collider, Sensor};

#[derive(Component)]
pub struct AreaOfEffect;

#[derive(Bundle)]
pub struct AreaOfEffectBundle {
    aoe_tag: AreaOfEffect,
    sensor_tag: Sensor,
    collider: Collider,
    transform_bundle: TransformBundle,
}
