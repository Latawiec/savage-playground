
use bevy::prelude::{Plugin, Vec2};
use bevy_rapier2d::prelude::{RapierPhysicsPlugin, NoUserData, RapierConfiguration};

use crate::game::aggro::plugin::AggroPlugin;
use crate::game::collision::plugin::ProbedRigidBodyPlugin;
use crate::game::lifetime::plugin::SelfDestructPlugin;
use crate::game::player::plugin::PlayerSystemsPlugin;
use crate::game::rendering::plugin::HeadlessRendererPlugin;
use crate::io::plugin::IOPlugin;

#[derive(Default)]
pub struct FrameworkPlugin;
impl Plugin for FrameworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        // Rapier:
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(0.01))
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..Default::default()
            })
        // Framework plugins:
            .add_plugins(AggroPlugin)
            .add_plugins(ProbedRigidBodyPlugin)
            .add_plugins(PlayerSystemsPlugin)
            .add_plugins(SelfDestructPlugin)
            .add_plugins(HeadlessRendererPlugin)
            .add_plugins(IOPlugin)
        ;

        // Debug features:
        #[cfg(feature = "debug_collider_renderer")]
        {
            use crate::game::debug::rapier_debug::RapierDebugViewPlugin;
            app.add_plugins(RapierDebugViewPlugin);
        }

        #[cfg(feature = "debug_local_input")]
        {
            use crate::game::debug::local_input::LocalInputPlugin;
            app.add_plugins(LocalInputPlugin);
        }

        #[cfg(feature="debug_diagnostics")]
        {
            use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
            use bevy::diagnostic::LogDiagnosticsPlugin;
            app.add_plugins(LogDiagnosticsPlugin::default());
            app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        }
    }
}
