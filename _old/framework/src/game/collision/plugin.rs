use bevy::prelude::{Plugin, PostUpdate, IntoSystemConfigs};
use bevy_rapier2d::prelude::PhysicsSet;

use super::system::{probed_rigid_body_probe_system, probed_rigid_body_fixup_system};



#[derive(Default)]
pub struct ProbedRigidBodyPlugin;
impl Plugin for ProbedRigidBodyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(
                PostUpdate,
                probed_rigid_body_probe_system.before(PhysicsSet::SyncBackend)
            )
            .add_systems(
                PostUpdate,
                probed_rigid_body_fixup_system.after(PhysicsSet::Writeback)
            );
            
    }
}