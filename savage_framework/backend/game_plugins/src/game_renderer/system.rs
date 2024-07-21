use bevy::prelude::{EventWriter, Query};
use game_renderer::proto::game_renderer::{SceneElement, UpdateType};

use super::{component::{RenderComponent, SceneElementId}, event::SceneElementsBatchEvent};

pub fn scene_full_update_system(
    mut ev_writer: EventWriter<SceneElementsBatchEvent>,
    query_scene_elements: Query<(&RenderComponent, &SceneElementId)>,
) {
    let mut scene_elements = Vec::default();
    for (render_component, render_id) in query_scene_elements.iter() {
        scene_elements.push(SceneElement {
            id: render_id.id(),
            update_type: Some(UpdateType::Full.into()),
            draw_bundle: Some(render_component.full_bundle()),
        });
    }
    ev_writer.send(SceneElementsBatchEvent(scene_elements));
}