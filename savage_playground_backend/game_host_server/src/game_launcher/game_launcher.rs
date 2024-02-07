use std::path::{Path, PathBuf};

use super::{
    error::GameLauncherError,
    game_instance::game_instance::GameInstance,
    game_mapping::GameMapping,
};

pub struct GameLauncher {
    games_root_directory: PathBuf,
    _games_mapping_file: PathBuf,
    game_mapping: GameMapping,
}

impl GameLauncher {
    pub fn new(
        games_root_directory: &Path,
        games_mapping_file: &Path,
    ) -> Result<GameLauncher, GameLauncherError> {
        let games_root_directory = games_root_directory.to_owned();
        let games_mapping_file = games_mapping_file.to_owned();
        let game_mapping = GameMapping::new(&games_mapping_file)?;

        Ok(GameLauncher {
            games_root_directory,
            _games_mapping_file: games_mapping_file,
            game_mapping,
        })
    }

    pub fn launch_game(
        &self,
        game_id: &str,
        args: &[String],
    ) -> Result<GameInstance, GameLauncherError> {
        let game_info = self.game_mapping.get_game_info(game_id)?;
        let game_cwd = self.games_root_directory.join(game_info.cwd);
        let game_exe = self.games_root_directory.join(game_info.exe);

        GameInstance::new(&game_cwd, &game_exe, args)
    }
}
