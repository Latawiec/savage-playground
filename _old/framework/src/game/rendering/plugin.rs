use bevy::prelude::{Plugin, PostUpdate};

use super::{event::RendererSnapshot, system::render_snapshot_system};

#[derive(Default)]
pub struct HeadlessRendererPlugin;
impl Plugin for HeadlessRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<RendererSnapshot>()
            .add_systems(PostUpdate, render_snapshot_system);
    }
}
