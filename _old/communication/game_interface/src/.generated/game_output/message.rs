#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameOutputMessage {
    #[prost(message, optional, tag = "1")]
    pub renderer: ::core::option::Option<super::renderer::RendererSnapshot>,
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<super::settings::SettingsSnapshot>,
    #[prost(message, optional, tag = "3")]
    pub ui: ::core::option::Option<super::ui::UiSnapshot>,
}
