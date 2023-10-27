use bevy::prelude::Component;
use serde::Serialize;

use crate::game::rendering::types::{
    blending::BlendingMode,
    layers::Layer,
    shader::{UniformProperties, VertexAttributes},
};

type Map<K, V> = std::collections::BTreeMap<K, V>;

#[derive(Default, Serialize)]
pub struct Assets {
    vertex_shader_path: String,
    pixel_shader_path: String,
    mesh_path: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    textures_paths: Map<u32, String>,
}

#[derive(Component, Default, Serialize)]
pub struct Drawable {
    assets: Assets,
    uniform_properties: UniformProperties,
    vertex_attributes: VertexAttributes,
    blending: BlendingMode,
    layer: Layer,
    billboard_xyz: (bool, bool, bool),
}

#[derive(Component, Default, Serialize)]
pub struct Sprite {
    pub texture_index: u32,
    pub columns: u32,
    pub rows: u32,
    pub active_tile: (u32, u32),
}
