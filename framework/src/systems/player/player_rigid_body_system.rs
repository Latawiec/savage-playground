use bevy::prelude::{Entity, FixedTime, GlobalTransform, Query, Res, Transform, Vec2, Vec3};
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, TOIStatus};

use crate::components::collision::player::PlayerRigidBody;

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
        let shape_vel = player_rigid_body.velocity;

        let shape_pos = Vec2::new(global_translation.x, global_translation.y);
        let shape_rot = global_rotation_euler.2;
        let shape = collider;
        
        // Check if we're not already penetrating other collider.
        // TODO: How?... Check rapier Contacts. And implementation of KinematicCharacterController. It uses Impulse somehow to push player back.
        //let contacts = rapier_context.contacts_with(entity);

        if let Some((_, toi)) = rapier_context.cast_shape(
            shape_pos,
            shape_rot,
            shape_vel,
            shape,
            delta_time,
            shape_cast_query_filter,
        ) {
            let collision_proj = toi.normal1 * shape_vel.dot(toi.normal1);
            let collision_rej = shape_vel - collision_proj;

            // Avoid penetration. If we're gonna hit in less than a frame, we won't be moving.
            let toi_offset = delta_time;

            let collision_offset = 0.1;
            if toi.status == TOIStatus::Penetrating {
                // Push off if penetrating, but it also allows free movement (in case they need to get out)
                let push_off = toi.normal1.normalize() * (shape_vel.dot(toi.normal1) + collision_offset) * delta_time;
                local_transform.translation += Vec3::new(push_off.x, push_off.y, 0.0);
                // println!("Toi: {:?}", toi);
            }

            let motion_update = (collision_rej * delta_time)
                + (collision_proj * f32::min((toi.toi - toi_offset).clamp(0.0, 1.0), delta_time));

            local_transform.translation += Vec3::new(motion_update.x, motion_update.y, 0.0);
        } else {
            local_transform.translation +=
                Vec3::new(shape_vel.x, shape_vel.y, 0.0) * delta_time;
        }
    }
}
