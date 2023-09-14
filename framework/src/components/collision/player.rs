use bevy::ecs::component::Component;
use bevy::prelude::{Entity, GlobalTransform, Query, Res, Transform, Vec2, Vec3};
use bevy::time::fixed_timestep::FixedTime;
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, TOIStatus, Vect};

/// Player Rigid Body works similarly to bevy RigidBody::KinematicVelocityBased.
/// The only difference is that [PlayerRigidBody] reacts to environment.
/// If a solid wall moves towards [PlayerRigidBody], [PlayerRigidBody] will be pushed without any impact on the wall.
#[derive(Component, Default)]
pub struct PlayerRigidBody {
    pub velocity: Vect,
}

pub fn player_rigid_body_system(
    fixed_time: Res<FixedTime>,
    rapier_context: Res<RapierContext>,
    mut player_rigid_bodies: Query<(
        Entity,
        &PlayerRigidBody,
        &Collider,        // If there's no collider, we can't check interactions.
        &GlobalTransform, // If there's no global transform, we can't check interaction.
        &mut Transform,   // If there's no transform, we can't check interaction.
    )>,
) {
    for (entity, player_rigid_body, collider, global_transform, mut local_transform) in
        player_rigid_bodies.iter_mut()
    {
        let global_transform_compute = global_transform.compute_transform();
        let global_translation = global_transform_compute.translation;
        let global_rotation_euler = global_transform_compute
            .rotation
            .to_euler(bevy::prelude::EulerRot::XYZ);

        // TODO: Make it smarter to use [collision_groups]
        let shape_cast_query_filter = QueryFilter::default()
            .exclude_collider(entity) // Exclude self.
            .exclude_sensors(); // Only consider Solids.

        let delta_time = fixed_time.period.as_secs_f32();
        let player_velocity = player_rigid_body.velocity;

        if let Some((_, toi)) = rapier_context.cast_shape(
            Vec2::new(global_translation.x, global_translation.y),
            global_rotation_euler.2,
            player_rigid_body.velocity,
            &collider,
            delta_time,
            shape_cast_query_filter,
        ) {
            let collision_proj = toi.normal1 * player_velocity.dot(toi.normal1);
            let collision_rej = player_velocity - collision_proj;

            // Avoid penetration. If we're gonna hit in less than a frame, we won't be moving.
            let toi_offset = delta_time;

            // If we have penetration, we have to fix it.
            // We can't entirely avoid penetration - for example if wall pushes the player, it is not player's input that causes penetration.
            let collision_offset = 0.1;
            if toi.status == TOIStatus::Penetrating {
                let push_off = toi.witness2.normalize() * collision_offset;
                local_transform.translation += Vec3::new(push_off.x, push_off.y, 0.0);
            }

            let motion_update = (collision_rej * delta_time)
                + (collision_proj * f32::min((toi.toi - toi_offset).clamp(0.0, 1.0), delta_time));

            local_transform.translation += Vec3::new(motion_update.x, motion_update.y, 0.0);
        } else {
            local_transform.translation +=
                Vec3::new(player_velocity.x, player_velocity.y, 0.0) * delta_time;
        }
    }
}
