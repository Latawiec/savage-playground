#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiSnapshot {
    #[prost(string, tag = "1")]
    pub type_desc: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<::prost_types::Any>,
}
