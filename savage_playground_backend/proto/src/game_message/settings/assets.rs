#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assets {
    #[prost(string, optional, tag = "1")]
    pub assets_package_path: ::core::option::Option<::prost::alloc::string::String>,
}
