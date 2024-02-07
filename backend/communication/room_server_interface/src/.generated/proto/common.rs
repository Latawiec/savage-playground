#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientId {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
