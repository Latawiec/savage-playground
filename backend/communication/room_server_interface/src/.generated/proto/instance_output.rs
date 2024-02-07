#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceOutput {
    #[prost(message, optional, tag = "1")]
    pub instance_output_msg: ::core::option::Option<::prost_types::Any>,
}
