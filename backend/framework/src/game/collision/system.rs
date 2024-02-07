use bevy::prelude::{FixedTime, Parent, Query, Res, Transform, Vec3, With, Without};

use super::component::probed_rigid_body::{PhysicsProbe, ProbedRigidBody};

// Runs before Physics update. Sets probe position at entity desired motion target (might be invalid, fixed by Physics update)
pub fn probed_rigid_body_probe_system(
    fixed_time: Res<FixedTime>,
    mut player_rigid_bodies: Query<(&ProbedRigidBody, &mut Transform)>,
) {
    for (rigid_body, mut transform) in &mut player_rigid_bodies {
        transform.translation += Vec3::new(rigid_body.velocity.x, rigid_body.velocity.y, 0.0)
            * fixed_time.period.as_secs_f32();
    }
}

// Runs after Physics update. Matches entity position with it's collision probe.
pub fn probed_rigid_body_fixup_system(
    mut player_rigid_bodies: Query<(&mut Transform, With<ProbedRigidBody>, Without<PhysicsProbe>)>,
    mut physics_probes: Query<(&Parent, &mut Transform, With<PhysicsProbe>)>,
) {
    for (probe_parent, mut probe_transform, _) in physics_probes.iter_mut() {
        match player_rigid_bodies.get_mut(probe_parent.get()) {
            Err(e) => {
                tracing::warn!("Failed to get Probe's parent ProbedRigidBody. {e}");
                return;
            }
            Ok((mut rigid_body_transform, _, _)) => {
                *rigid_body_transform = rigid_body_transform.mul_transform(*probe_transform);
                *probe_transform = Transform::default();
            }
        }
    }
}
