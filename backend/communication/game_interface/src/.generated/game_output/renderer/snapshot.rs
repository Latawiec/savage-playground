#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub drawable: ::core::option::Option<super::drawable::Drawable>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Camera {
    #[prost(message, optional, tag = "1")]
    pub view: ::core::option::Option<super::gl_types::FloatArray>,
    #[prost(message, optional, tag = "2")]
    pub proj: ::core::option::Option<super::gl_types::FloatArray>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    #[prost(enumeration = "UpdateType", optional, tag = "1")]
    pub update_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateType {
    Reset = 0,
    Increment = 1,
}
impl UpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateType::Reset => "Reset",
            UpdateType::Increment => "Increment",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Reset" => Some(Self::Reset),
            "Increment" => Some(Self::Increment),
            _ => None,
        }
    }
}
