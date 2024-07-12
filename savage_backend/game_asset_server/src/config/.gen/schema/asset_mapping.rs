use serde::{Deserialize, Serialize};
/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///Maps asset alias name to correct host-side asset file paths.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "/savage_playground/game_host_server/asset_mapping.json",
///  "title": "AssetMapping",
///  "description": "Maps asset alias name to correct host-side asset file paths.",
///  "type": "object",
///  "additionalProperties": {
///    "type": "string"
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetMapping(pub std::collections::HashMap<String, String>);
impl std::ops::Deref for AssetMapping {
    type Target = std::collections::HashMap<String, String>;
    fn deref(&self) -> &std::collections::HashMap<String, String> {
        &self.0
    }
}
impl From<AssetMapping> for std::collections::HashMap<String, String> {
    fn from(value: AssetMapping) -> Self {
        value.0
    }
}
impl From<&AssetMapping> for AssetMapping {
    fn from(value: &AssetMapping) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, String>> for AssetMapping {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        Self(value)
    }
}
