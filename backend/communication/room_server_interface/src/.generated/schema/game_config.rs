use serde::{Deserialize, Serialize};
///Game configuration.
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
        builder::GameConfig::default()
    }
}
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
            self
                .game_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for game_id: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<GameConfig> for super::GameConfig {
        type Error = String;
        fn try_from(value: GameConfig) -> Result<Self, String> {
            Ok(Self { game_id: value.game_id? })
        }
    }
    impl From<super::GameConfig> for GameConfig {
        fn from(value: super::GameConfig) -> Self {
            Self { game_id: Ok(value.game_id) }
        }
    }
}
