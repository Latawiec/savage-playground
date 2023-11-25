#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientId {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Origin: Client
/// Target: Instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMessage {
    #[prost(message, optional, tag = "1")]
    pub client_id: ::core::option::Option<ClientId>,
    #[prost(message, optional, tag = "2")]
    pub game_input_message: ::core::option::Option<::prost_types::Any>,
}
/// Origin: Instance
/// Target: Client
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceDirectMessage {
    #[prost(message, optional, tag = "1")]
    pub client_id: ::core::option::Option<ClientId>,
    #[prost(message, optional, tag = "2")]
    pub game_output_message: ::core::option::Option<::prost_types::Any>,
}
/// Origin: Instance
/// Target: All Clients
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceBroadcast {
    #[prost(message, optional, tag = "1")]
    pub game_output_message: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceMessage {
    #[prost(message, repeated, tag = "1")]
    pub direct_messages: ::prost::alloc::vec::Vec<InstanceDirectMessage>,
    #[prost(message, optional, tag = "2")]
    pub broadcast: ::core::option::Option<InstanceBroadcast>,
}
