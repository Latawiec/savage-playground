#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexAttributes {
    #[prost(string, optional, tag = "1")]
    pub vertices: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(map = "string, string", tag = "2")]
    pub named_buffers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
