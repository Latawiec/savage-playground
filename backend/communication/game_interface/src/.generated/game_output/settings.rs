#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assets {
    #[prost(string, optional, tag = "1")]
    pub assets_package_path: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Window {
    #[prost(float, optional, tag = "1")]
    pub aspect_ratio: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "2")]
    pub rt_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub rt_height: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingsSnapshot {
    #[prost(message, optional, tag = "1")]
    pub assets: ::core::option::Option<Assets>,
    #[prost(message, optional, tag = "2")]
    pub window: ::core::option::Option<Window>,
}
