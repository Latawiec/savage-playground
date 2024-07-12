
#[derive(Debug)]
pub enum GameLauncherError {
    GameMappingFileError { reason: String },
    GameMappingFileIllFormed { reason: String},
    GameNotFound { game_id: String },

    InstanceStartupError { reason: String },
    InstanceKillError { reason: String },
}