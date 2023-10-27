use bevy::prelude::Event;

#[derive(Event, Default)]
pub struct RendererSnapshot {
    pub snapshot: serde_json::Value,
}
