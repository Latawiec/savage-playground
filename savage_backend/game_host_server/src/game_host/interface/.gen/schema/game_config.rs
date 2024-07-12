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
///Game configuration.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "/savage_playground/host_interface/game_config.json",
///  "title": "GameConfig",
///  "description": "Game configuration.",
///  "type": "object",
///  "properties": {
///    "game_id": {
///      "description": "Path for the game to start. Make sure game exists in game mapping file on the server.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameConfig {
    ///Path for the game to start. Make sure game exists in game mapping file on the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
}
impl From<&GameConfig> for GameConfig {
    fn from(value: &GameConfig) -> Self {
        value.clone()
    }
}
impl GameConfig {
    pub fn builder() -> builder::GameConfig {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct GameConfig {
        game_id: Result<Option<String>, String>,
    }
    impl Default for GameConfig {
        fn default() -> Self {
            Self {
                game_id: Ok(Default::default()),
            }
        }
    }
    impl GameConfig {
        pub fn game_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.game_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for game_id: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<GameConfig> for super::GameConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: GameConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self { game_id: value.game_id? })
        }
    }
    impl From<super::GameConfig> for GameConfig {
        fn from(value: super::GameConfig) -> Self {
            Self { game_id: Ok(value.game_id) }
        }
    }
}
