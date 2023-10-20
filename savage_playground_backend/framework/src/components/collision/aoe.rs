use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use bevy::transform::TransformBundle;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{Real, Sensor, CollisionGroups, Group};

use super::collision_groups::*;

#[derive(Default, Component)]
pub struct AreaOfEffect;

#[derive(Default, Bundle)]
pub struct AreaOfEffectBundle {
    collider: Collider,
    sensor_tag: Sensor,
    aoe_tag: AreaOfEffect,
    transform_bundle: TransformBundle,
    collision_groups: CollisionGroups,
}

impl AreaOfEffectBundle {
    fn collision_groups() -> CollisionGroups {
        CollisionGroups {
            memberships: Group::from(Group::from_bits_truncate(SENSOR_AOE_HITBOX)),
            filters: Group::from(Group::from_bits_truncate(SENSOR_PLAYER_HITBOX)),
        }
    }

    pub fn set_transform(mut self, transform: TransformBundle) -> AreaOfEffectBundle {
        self.transform_bundle = transform;
        self
    }

    pub fn circle(radius: f32) -> AreaOfEffectBundle {
        AreaOfEffectBundle {
            collider: Collider::ball(radius),
            collision_groups: Self::collision_groups(),
            ..Default::default()
        }
    }

    pub fn donut(inner_radius: f32, outer_radius: f32, segments: u32) -> AreaOfEffectBundle {
        // For now simple segments, but outter circle can have as many vercies as I want for more resolution.
        let mut colliders = Vec::<Collider>::new();
        colliders.reserve(segments as usize);

        for i in 0..segments {
            let start_angle = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
            let end_angle = 2.0 * std::f32::consts::PI * (i + 1) as f32 / segments as f32;

            let start_point = Vec2::new(start_angle.sin(), start_angle.cos());
            let end_point = Vec2::new(end_angle.sin(), end_angle.cos());

            let points = vec![
                start_point * outer_radius, // Top-left
                end_point * outer_radius,   // Top-right
                end_point * inner_radius,   // Bottom-right
                start_point * inner_radius, // Bottom-left
            ];

            let collider = Collider::convex_polyline(points)
                .expect("Points passed to create a convex polyline were invalid.");
            colliders.push(collider);
        }

        AreaOfEffectBundle {
            collider: Collider::compound(
                colliders
                    .into_iter()
                    .map(|shape| (Vec2::ZERO, Real::default(), shape))
                    .collect(),
            ),
            collision_groups: Self::collision_groups(),
            ..Default::default()
        }
    }

    pub fn rectangle(half_width: f32, half_height: f32) -> AreaOfEffectBundle {
        AreaOfEffectBundle {
            collider: Collider::cuboid(half_width, half_height),
            ..Default::default()
        }
    }

    // Basically rectangle, but anchored at the bottom side
    pub fn beam(half_width: f32, distance: f32) -> AreaOfEffectBundle {
        let half_distance = distance / 2.0;

        AreaOfEffectBundle {
            // Compound just to have Isometry to it. Move it up by half distance.
            collider: Collider::compound(vec![(
                Vec2::new(0.0, half_distance),
                Real::default(),
                Collider::cuboid(half_width, half_distance),
            )]),
            collision_groups: Self::collision_groups(),
            ..Default::default()
        }
    }

    pub fn inverse_cone(distance: f32) -> AreaOfEffectBundle {

        let left = Vec2::new(-1.0, 0.0) * distance;
        let right = Vec2::new(1.0, 0.0) * distance;
        let forward = Vec2::new(0.0, 1.0) * distance;

        let collider = Collider::convex_polyline(vec![
            left,
            forward,
            right
        ])
        .expect("Points passed to create a convex polyline were invalid.");

        AreaOfEffectBundle {
            collider,
            collision_groups: Self::collision_groups(),
            ..Default::default()
        }
    }

    // Basically a triangle.
    pub fn cone(spread_radians: f32, distance: f32) -> AreaOfEffectBundle {
        // If a triangle is spread too wide, I loose control over how high it can be. Set a safety net.
        debug_assert!(spread_radians < std::f32::consts::PI * 0.8);

        let half_spread = spread_radians / 2.0;

        let left_dir = Vec2::new(-half_spread.sin(), half_spread.cos());
        let right_dir = Vec2::new(half_spread.sin(), half_spread.cos());

        // Need to make arms length longer so that straight line distance from start to end is {distance}
        let arms_length = distance / half_spread.cos();

        let collider = Collider::convex_polyline(vec![
            Vec2::ZERO,
            left_dir * arms_length,
            right_dir * arms_length,
        ])
        .expect("Points passed to create a convex polyline were invalid.");

        AreaOfEffectBundle {
            collider,
            collision_groups: Self::collision_groups(),
            ..Default::default()
        }
    }

    // Pie-chart like shape. Full circle at PI spread.
    pub fn cleave(spread_radians: f32, distance: f32, segments: u32) -> AreaOfEffectBundle {
        // More than 2PI spread starts overlaping. 2PI is full circle. Assume it's a bug if someone tries it.
        debug_assert!(spread_radians < std::f32::consts::PI * 2.0);

        // TODO: Make it smarter. Merge bigger shapes
        // Right now I'll just do {segments} of triangles.
        let mut colliders = Vec::<Collider>::new();
        colliders.reserve(segments as usize);

        let half_spread = spread_radians / 2.0;
        let spread_step = spread_radians / segments as f32;

        for i in 0..segments {
            let start_angle = -half_spread + i as f32 * spread_step;
            let end_angle = -half_spread + (i + 1) as f32 * spread_step;

            let start_point = Vec2::new(start_angle.sin(), start_angle.cos());
            let end_point = Vec2::new(end_angle.sin(), end_angle.cos());

            colliders.push(
                Collider::convex_polyline(vec![
                    Vec2::ZERO,
                    start_point * distance,
                    end_point * distance,
                ])
                .expect("Points passed to create a convex polyline were invalid."),
            );
        }

        AreaOfEffectBundle {
            collider: Collider::compound(
                colliders
                    .into_iter()
                    .map(|collider| (Vec2::ZERO, Real::default(), collider))
                    .collect(),
            ),
            collision_groups: Self::collision_groups(),
            ..Default::default()
        }
    }
}
