use bevy::prelude::{Entity, FixedTime, Query, Res, Transform, Vec3, With, Without};

use crate::components::collision::probed_rigid_body::{PhysicsProbe, ProbedRigidBody};

pub fn post_physics_update(
    mut player_rigid_bodies: Query<(&ProbedRigidBody, &mut Transform, Without<PhysicsProbe>)>,
    mut physics_probes: Query<(Entity, &mut Transform, With<PhysicsProbe>)>,
) {
    for (rigid_body, mut main_transform, _) in player_rigid_bodies.iter_mut() {
        match physics_probes.get_mut(rigid_body.probe) {
            Err(e) => {
                tracing::warn!("Error fetching Probe for ProbedRigidBody: {e}");
                return;
            }
            Ok((_, mut probe_transform, _)) => {
                *main_transform = main_transform.mul_transform(*probe_transform);
                *probe_transform = Transform::default();
            }
        }
    }
}

pub fn pre_physics_update(
    fixed_time: Res<FixedTime>,
    mut player_rigid_bodies: Query<(&ProbedRigidBody, &mut Transform)>,
) {
    for (rigid_body, mut transform) in &mut player_rigid_bodies {
        transform.translation += Vec3::new(rigid_body.velocity.x, rigid_body.velocity.y, 0.0)
            * fixed_time.period.as_secs_f32();
    }
}
