use super::error::GameLauncherError;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Clone)]
pub struct GameInfo {
    pub cwd: PathBuf,
    pub exe: PathBuf,
}

pub struct GameMapping {
    games_mapping: BTreeMap<String, GameInfo>,
}

impl GameMapping {
    pub fn new(game_mapping_file: &Path) -> Result<GameMapping, GameLauncherError> {
        let games_mapping_json = match fs::read_to_string(game_mapping_file) {
            Ok(file_content) => file_content,
            Err(error) => {
                return Err(GameLauncherError::GameMappingFileError {
                    reason: error.to_string(),
                })
            }
        };

        let games_mapping =
            match serde_json::from_str::<BTreeMap<String, GameInfo>>(&games_mapping_json) {
                Ok(deserialized_content) => deserialized_content,
                Err(error) => {
                    return Err(GameLauncherError::GameMappingFileIllFormed {
                        reason: error.to_string(),
                    })
                }
            };

        Ok(GameMapping { games_mapping })
    }

    pub fn get_game_info(&self, game_id: &str) -> Result<GameInfo, GameLauncherError> {
        if let Some(game_info) = self.games_mapping.get(game_id) {
            Ok(game_info.to_owned())
        } else {
            Err(GameLauncherError::GameNotFound {
                game_id: game_id.to_owned(),
            })
        }
    }
}
