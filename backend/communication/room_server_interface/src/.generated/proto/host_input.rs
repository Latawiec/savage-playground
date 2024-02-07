#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostInput {
    #[prost(message, optional, tag = "1")]
    pub joined: ::core::option::Option<super::common::ClientId>,
    #[prost(message, optional, tag = "2")]
    pub left: ::core::option::Option<super::common::ClientId>,
}
