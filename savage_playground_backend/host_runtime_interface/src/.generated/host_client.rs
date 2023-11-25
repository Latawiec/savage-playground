/// Origin: Client
/// Target: Host
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMessage {
    #[prost(message, optional, tag = "1")]
    pub game_input_message: ::core::option::Option<::prost_types::Any>,
}
/// Origin: Host
/// Target: Client
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostMessage {
    #[prost(message, optional, tag = "1")]
    pub game_output_message: ::core::option::Option<::prost_types::Any>,
}
