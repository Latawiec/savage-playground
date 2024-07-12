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
///GameInfo
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cwd",
///    "exe"
///  ],
///  "properties": {
///    "args": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "cwd": {
///      "type": "string"
///    },
///    "exe": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    pub cwd: String,
    pub exe: String,
}
impl From<&GameInfo> for GameInfo {
    fn from(value: &GameInfo) -> Self {
        value.clone()
    }
}
impl GameInfo {
    pub fn builder() -> builder::GameInfo {
        Default::default()
    }
}
///Maps game alias name to correct host-side game info.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "/savage_playground/game_host_server/game_mapping.json",
///  "title": "GameMapping",
///  "description": "Maps game alias name to correct host-side game info.",
///  "type": "object",
///  "additionalProperties": {
///    "$ref": "#/$defs/game_info"
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameMapping(pub std::collections::HashMap<String, GameInfo>);
impl std::ops::Deref for GameMapping {
    type Target = std::collections::HashMap<String, GameInfo>;
    fn deref(&self) -> &std::collections::HashMap<String, GameInfo> {
        &self.0
    }
}
impl From<GameMapping> for std::collections::HashMap<String, GameInfo> {
    fn from(value: GameMapping) -> Self {
        value.0
    }
}
impl From<&GameMapping> for GameMapping {
    fn from(value: &GameMapping) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, GameInfo>> for GameMapping {
    fn from(value: std::collections::HashMap<String, GameInfo>) -> Self {
        Self(value)
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct GameInfo {
        args: Result<Vec<String>, String>,
        cwd: Result<String, String>,
        exe: Result<String, String>,
    }
    impl Default for GameInfo {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                cwd: Err("no value supplied for cwd".to_string()),
                exe: Err("no value supplied for exe".to_string()),
            }
        }
    }
    impl GameInfo {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {}", e));
            self
        }
        pub fn cwd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.cwd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cwd: {}", e));
            self
        }
        pub fn exe<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.exe = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exe: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<GameInfo> for super::GameInfo {
        type Error = super::error::ConversionError;
        fn try_from(value: GameInfo) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                cwd: value.cwd?,
                exe: value.exe?,
            })
        }
    }
    impl From<super::GameInfo> for GameInfo {
        fn from(value: super::GameInfo) -> Self {
            Self {
                args: Ok(value.args),
                cwd: Ok(value.cwd),
                exe: Ok(value.exe),
            }
        }
    }
}
