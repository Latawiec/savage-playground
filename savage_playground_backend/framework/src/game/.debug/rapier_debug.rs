use bevy::prelude::{Camera2dBundle, Plugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

#[derive(Default)]
pub struct RapierDebugViewPlugin;
impl Plugin for RapierDebugViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.world.spawn(Camera2dBundle::default());
    }
}
