#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientOutput {
    #[prost(message, optional, tag = "1")]
    pub game_output_message: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMessage {
    #[prost(message, optional, tag = "1")]
    pub client_id: ::core::option::Option<super::common::ClientId>,
    #[prost(message, optional, tag = "2")]
    pub client_output: ::core::option::Option<ClientOutput>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomBroadcast {
    #[prost(message, optional, tag = "1")]
    pub client_output: ::core::option::Option<ClientOutput>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientOutputBatch {
    #[prost(message, repeated, tag = "1")]
    pub direct_messages: ::prost::alloc::vec::Vec<DirectMessage>,
    #[prost(message, optional, tag = "2")]
    pub broadcast: ::core::option::Option<RoomBroadcast>,
}
