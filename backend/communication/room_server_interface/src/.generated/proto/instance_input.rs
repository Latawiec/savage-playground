#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceInput {
    #[prost(message, optional, tag = "1")]
    pub instance_input_msg: ::core::option::Option<::prost_types::Any>,
}
