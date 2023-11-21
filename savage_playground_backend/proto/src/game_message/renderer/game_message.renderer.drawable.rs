#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Drawable {
    #[prost(uint32, optional, tag = "1")]
    pub layer: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub vertex_attributes: ::core::option::Option<
        super::vertex_attributes::VertexAttributes,
    >,
    #[prost(message, optional, tag = "3")]
    pub uniform_attributes: ::core::option::Option<
        super::uniform_attributes::UniformAttributes,
    >,
    #[prost(message, optional, tag = "4")]
    pub shared_attributes: ::core::option::Option<
        super::uniform_attributes::SharedUniformAttributes,
    >,
}
