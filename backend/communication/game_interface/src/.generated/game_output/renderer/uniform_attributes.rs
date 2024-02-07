#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniformAttributes {
    #[prost(map = "string, float", tag = "1")]
    pub float: ::std::collections::HashMap<::prost::alloc::string::String, f32>,
    #[prost(map = "string, message", tag = "2")]
    pub vec2: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::gl_types::FloatArray,
    >,
    #[prost(map = "string, message", tag = "3")]
    pub vec3: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::gl_types::FloatArray,
    >,
    #[prost(map = "string, message", tag = "4")]
    pub vec4: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::gl_types::FloatArray,
    >,
    #[prost(map = "string, uint32", tag = "5")]
    pub int: ::std::collections::HashMap<::prost::alloc::string::String, u32>,
    #[prost(map = "string, message", tag = "6")]
    pub ivec2: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::gl_types::Uint32Array,
    >,
    #[prost(map = "string, message", tag = "7")]
    pub ivec3: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::gl_types::Uint32Array,
    >,
    #[prost(map = "string, message", tag = "8")]
    pub ivec4: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::gl_types::Uint32Array,
    >,
    #[prost(map = "string, message", tag = "9")]
    pub mat4: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::gl_types::FloatArray,
    >,
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
