use super::proto_pipe::{ProtoStderr, ProtoStdin, ProtoStdout};
use crate::{game_launcher::error::GameLauncherError, instance::instance::Instance};
use std::path::Path;

pub struct GameInstance {
    _instance: Instance,
    pub stdin: Option<ProtoStdin>,
    pub stdout: Option<ProtoStdout>,
    pub stderr: Option<ProtoStderr>,
}

impl GameInstance {
    pub fn new(
        cwd: &Path,
        exe: &Path,
        _args: &[String],
    ) -> Result<GameInstance, GameLauncherError> {
        let instance_result = Instance::new(&cwd, &exe);

        if let Err(error) = &instance_result {
            return Err(GameLauncherError::InstanceStartupError {
                reason: error.to_string(),
            });
        }
        let mut instance = instance_result.unwrap();

        let stdin = instance.take_stdin().map(|stdin| ProtoStdin::new(stdin));
        let stdout = instance
            .take_stdout()
            .map(|stdout| ProtoStdout::new(stdout));
        let stderr = instance
            .take_stderr()
            .map(|stderr| ProtoStderr::new(stderr));

        Ok(GameInstance {
            _instance: instance,
            stdin,
            stdout,
            stderr,
        })
    }

    pub async fn kill(mut self) -> Result<(), GameLauncherError> {
        if let Err(error) = self._instance.kill().await {
            return Err(GameLauncherError::InstanceKillError {
                reason: error.to_string(),
            });
        }
        Ok(())
    }
}
