#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    #[prost(message, optional, tag = "1")]
    pub assets: ::core::option::Option<super::assets::Assets>,
    #[prost(message, optional, tag = "2")]
    pub window: ::core::option::Option<super::window::Window>,
}
