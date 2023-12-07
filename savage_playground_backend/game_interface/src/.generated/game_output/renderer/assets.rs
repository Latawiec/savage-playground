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
