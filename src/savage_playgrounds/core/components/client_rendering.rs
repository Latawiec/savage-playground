use bevy::prelude::{default, Component, GlobalTransform, Mat4, Query};
use serde::Serialize;
use serde_json;

type Map<K, V> = std::collections::BTreeMap<K, V>;

pub mod layers {
    pub const WAYMARKERS_LAYER: u32 = 14;
    pub const SUPPORT_LAYER: u32 = 13;
    pub const PLAYER_LAYER: u32 = 12;
    pub const AOE_LAYER: u32 = 11;
    pub const GROUND_LAYER: u32 = 10;
    pub const BACKGROUND_LAYER: u32 = 0;
}

#[derive(Serialize)]
pub enum BlendingMode {
    Transparency,
    Opaque,
    Additive,
}

mod gl_types {
    pub type float = f32;
    pub type vec2 = (float, float);
    pub type vec3 = (float, float, float);
    pub type vec4 = (float, float, float, float);
    pub type int = i32;
    pub type ivec2 = (int, int);
    pub type ivec3 = (int, int, int);
    pub type ivec4 = (int, int, int, int);
    pub type mat4 = (vec4, vec4, vec4, vec4);
}

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

#[derive(Serialize)]
pub struct Assets {
    vertex_shader_path: String,
    pixel_shader_path: String,
    mesh_path: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    textures_paths: Map<u32, String>,
}

#[derive(Component, Serialize)]
pub struct ClientDrawable {
    assets: Assets,
    uniform_properties: UniformProperties,
    vertex_attributes: VertexAttributes,
    blending: BlendingMode,
    billboard_xyz: (bool, bool, bool),
}

#[derive(Component, Serialize)]
pub struct ClientSprite {
    texture_index: u32,
    columns: u32,
    rows: u32,
    active_tile: (u32, u32),
}

#[derive(Serialize)]
struct Drawable<'a> {
    transform: [f32; 16],
    #[serde(flatten)]
    _drawable: &'a ClientDrawable,
    #[serde(skip_serializing_if = "Option::is_none")]
    sprite: Option<&'a ClientSprite>,
}

pub fn serialize_client_drawable(
    client_drawables: Query<(&GlobalTransform, &ClientDrawable, Option<&ClientSprite>)>,
) {
    for (global_transform, client_drawable, client_sprite_opt) in client_drawables.iter() {
        let drawable = Drawable {
            transform: global_transform
                .compute_transform()
                .compute_matrix()
                .to_cols_array(),
            _drawable: client_drawable,
            sprite: client_sprite_opt,
        };

        println!("{:?}", serde_json::to_string(&drawable));
    }
}

#[test]
fn check_serialize_client_drawable() {
    let client_drawable = ClientDrawable {
        assets: Assets {
            vertex_shader_path: "Vs".to_string(),
            pixel_shader_path: "Ps".to_string(),
            mesh_path: "Mesh".to_string(),
            textures_paths: Map::from([(0, "ZeroText".to_string()), (1, "OneText".to_string())]),
        },
        uniform_properties: UniformProperties {
            vec4: Map::from([("transform".to_string(), (0.0, 0.0, 0.0, 0.0))]),
            ..Default::default()
        },
        vertex_attributes: VertexAttributes {
            vertices: "aVertexPointer".to_string(),
            uv_1: Some("uv_map_11".to_string()),
            ..Default::default()
        },
        blending: BlendingMode::Opaque,
        billboard_xyz: (false, true, false),
    };

    let client_sprite = ClientSprite {
        texture_index: 1,
        columns: 3,
        rows: 4,
        active_tile: (0, 1),
    };

    let transform = Mat4::IDENTITY.to_cols_array();

    let drawable = Drawable {
        transform,
        _drawable: &client_drawable,
        sprite: Some(&client_sprite),
    };

    println!("{}", serde_json::to_string_pretty(&drawable).unwrap());
}
