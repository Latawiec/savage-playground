use bevy::prelude::Component;
use serde::Serialize;

use super::layers::Layer;
use super::blending::BlendingMode;
use super::shader::{ UniformProperties, VertexAttributes };

type Map<K, V> = std::collections::BTreeMap<K, V>;

#[derive(Serialize)]
pub struct Assets {
    vertex_shader_path: String,
    pixel_shader_path: String,
    mesh_path: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    textures_paths: Map<u32, String>,
}

#[derive(Component, Serialize)]
pub struct Drawable {
    assets: Assets,
    uniform_properties: UniformProperties,
    vertex_attributes: VertexAttributes,
    blending: BlendingMode,
    layer: Layer,
    billboard_xyz: (bool, bool, bool),
}

#[derive(Component, Serialize)]
pub struct Sprite {
    texture_index: u32,
    columns: u32,
    rows: u32,
    active_tile: (u32, u32),
}