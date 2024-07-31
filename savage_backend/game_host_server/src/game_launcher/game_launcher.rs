use std::path::{Path, PathBuf};

use super::{
    error::GameLauncherError,
    game_instance::game_instance::GameInstance,
    game_mapping::{GameInfo, GameMapping},
};

/// Game launcher is used to start up games that can communicate with the server.
pub struct GameLauncher {
    games_root_directory: PathBuf,
    _games_mapping_file: PathBuf,
    game_mapping: GameMapping,
}

impl GameLauncher {
    /// Creates a new Game Launcher.
    /// 
    /// * `game_root_directory` - base directory to look for games.
    /// * `games_mapping_file` - file describing games mapping. Any game that you want to be able to
    /// launch by the GameLauncher should be defined in this file. To see the file format check
    /// `crate::GameMapping`.
    pub fn new(
        games_root_directory: &Path,
        games_mapping_file: &Path,
    ) -> Result<GameLauncher, GameLauncherError> {
        let games_root_directory = games_root_directory.to_owned();
        let games_mapping_file = games_mapping_file.to_owned();
        let game_mapping = GameMapping::new(&games_mapping_file)?;

        tracing::info!(name: "game_launcher", root_dir = format!("{:?}", games_root_directory), mapping_file = format!("{:?}", games_mapping_file), "created");

        Ok(GameLauncher {
            games_root_directory,
            _games_mapping_file: games_mapping_file,
            game_mapping,
        })
    }

    /// Launches a game.
    /// 
    /// Remember that only games registered when creating the GameLauncher can be launched.
    /// 
    /// * `game_id` - name of the game (defined by game_mapping_file contents) to be launched.
    /// * `args` - arguments to be passed to the game on launch.
    pub fn launch_game(
        &self,
        game_id: &str,
        args: &[&str],
    ) -> Result<GameInstance, GameLauncherError> {
        let game_info = self.game_mapping.get_game_info(game_id)?;
        let game_cwd = self.games_root_directory.join(game_info.cwd);
        let game_exe = self.games_root_directory.join(game_info.exe);
        let game_args = { 
            let mut result = game_info.args.unwrap_or(Vec::new());
            result.extend(args.iter().map(|&str| str.to_owned()));
            result
        };
        tracing::info!(name: "game_launcher", game_id, cwd = format!("{:?}", game_cwd), exe = format!("{:?}", game_exe), args = format!("{:?}", game_args), "game is launching");
        
        match GameInstance::new(&game_cwd, &game_exe, &game_args) {
            Ok(ok) => Ok(ok),
            Err(err) => {
                tracing::error!(name: "game_launcher", game_id, cwd = format!("{:?}", game_cwd), exe = format!("{:?}", game_exe), args = format!("{:?}", game_args), "game failed to launch: {:?}", err);
                Err(err)
            },
        }
    }

    pub fn get_game_info(
        &self,
        game_id: &str,
    ) -> Result<GameInfo, GameLauncherError> {
        Ok(self.game_mapping.get_game_info(game_id)?)
    }
}
