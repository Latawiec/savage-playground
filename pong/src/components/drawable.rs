use bevy::prelude::Component;
use serde::Serialize;

use super::gl_types;

type Map<K, V> = std::collections::HashMap<K, V>;

#[derive(Serialize, Default)]
pub struct UniformProperties {
    #[serde(skip_serializing_if = "Map::is_empty")]
    float: Map<String, gl_types::float>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    vec2: Map<String, gl_types::vec2>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    vec3: Map<String, gl_types::vec3>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    vec4: Map<String, gl_types::vec4>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    int: Map<String, gl_types::int>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    ivec2: Map<String, gl_types::ivec2>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    ivec3: Map<String, gl_types::ivec3>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    ivec4: Map<String, gl_types::ivec4>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    mat4: Map<String, gl_types::mat4>,
}

#[derive(Serialize, Default)]
pub struct VertexAttributes {
    vertices: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uv_0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uv_1: Option<String>,
}

#[derive(Default, Serialize)]
pub struct Assets {
    vertex_shader_path: String,
    pixel_shader_path: String,
    mesh_path: String,
}

#[derive(Component, Default, Serialize)]
pub struct Drawable {
    assets: Assets,
    uniform_properties: UniformProperties,
    vertex_attributes: VertexAttributes,
}