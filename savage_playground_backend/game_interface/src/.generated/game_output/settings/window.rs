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
