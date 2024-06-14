#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexAttributes {
    #[prost(string, optional, tag = "1")]
    pub vertices: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(map = "string, string", tag = "2")]
    pub named_buffers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatArray {
    #[prost(float, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint32Array {
    #[prost(uint32, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniformAttributes {
    #[prost(map = "string, float", tag = "1")]
    pub float: ::std::collections::HashMap<::prost::alloc::string::String, f32>,
    #[prost(map = "string, message", tag = "2")]
    pub vec2: ::std::collections::HashMap<::prost::alloc::string::String, FloatArray>,
    #[prost(map = "string, message", tag = "3")]
    pub vec3: ::std::collections::HashMap<::prost::alloc::string::String, FloatArray>,
    #[prost(map = "string, message", tag = "4")]
    pub vec4: ::std::collections::HashMap<::prost::alloc::string::String, FloatArray>,
    #[prost(map = "string, uint32", tag = "5")]
    pub int: ::std::collections::HashMap<::prost::alloc::string::String, u32>,
    #[prost(map = "string, message", tag = "6")]
    pub ivec2: ::std::collections::HashMap<::prost::alloc::string::String, Uint32Array>,
    #[prost(map = "string, message", tag = "7")]
    pub ivec3: ::std::collections::HashMap<::prost::alloc::string::String, Uint32Array>,
    #[prost(map = "string, message", tag = "8")]
    pub ivec4: ::std::collections::HashMap<::prost::alloc::string::String, Uint32Array>,
    #[prost(map = "string, message", tag = "9")]
    pub mat4: ::std::collections::HashMap<::prost::alloc::string::String, FloatArray>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraUniformAttributes {
    /// mat4
    #[prost(string, optional, tag = "1")]
    pub camera_view: ::core::option::Option<::prost::alloc::string::String>,
    /// mat4
    #[prost(string, optional, tag = "2")]
    pub camera_proj: ::core::option::Option<::prost::alloc::string::String>,
    /// vec3
    #[prost(string, optional, tag = "3")]
    pub camera_forward: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedUniformAttributes {
    #[prost(message, optional, tag = "1")]
    pub camera: ::core::option::Option<CameraUniformAttributes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Texture {
    #[prost(uint32, tag = "1")]
    pub offset: u32,
    #[prost(string, optional, tag = "2")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assets {
    #[prost(string, optional, tag = "1")]
    pub vertex_shader_asset: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub pixel_shader_asset: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub mesh_asset: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub textures: ::prost::alloc::vec::Vec<Texture>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Drawable {
    #[prost(uint32, optional, tag = "1")]
    pub layer: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub vertex_attributes: ::core::option::Option<VertexAttributes>,
    #[prost(message, optional, tag = "3")]
    pub uniform_attributes: ::core::option::Option<UniformAttributes>,
    #[prost(message, optional, tag = "4")]
    pub shared_attributes: ::core::option::Option<SharedUniformAttributes>,
    #[prost(message, optional, tag = "5")]
    pub assets: ::core::option::Option<Assets>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub drawable: ::core::option::Option<Drawable>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Camera {
    #[prost(message, optional, tag = "1")]
    pub view: ::core::option::Option<FloatArray>,
    #[prost(message, optional, tag = "2")]
    pub proj: ::core::option::Option<FloatArray>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RendererSnapshot {
    #[prost(enumeration = "UpdateType", optional, tag = "1")]
    pub update_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateType {
    Reset = 0,
    Increment = 1,
}
impl UpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateType::Reset => "Reset",
            UpdateType::Increment => "Increment",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Reset" => Some(Self::Reset),
            "Increment" => Some(Self::Increment),
            _ => None,
        }
    }
}
