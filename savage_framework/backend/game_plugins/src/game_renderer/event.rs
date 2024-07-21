use bevy::prelude::Event;
use game_renderer::proto::game_renderer::SceneElement;

#[derive(Event)]
pub struct SceneElementsBatchEvent(pub Vec<SceneElement>);