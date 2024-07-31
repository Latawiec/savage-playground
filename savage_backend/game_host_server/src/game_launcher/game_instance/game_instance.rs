use super::proto_pipe::{ProtoStderr, ProtoStdin, ProtoStdout};
use crate::{game_launcher::error::GameLauncherError, instance::{self, instance::Instance}};
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
        args: &[String],
    ) -> Result<GameInstance, GameLauncherError> {
        let instance_result = Instance::new(&cwd, &exe, &args);

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

    pub async fn wait(&mut self) -> Result<(), instance::instance::Error>{
        match self._instance.wait().await {
            Ok(exit_status) => {
                if exit_status.success() {
                    return Ok(())
                } else {
                    tracing::error!(name: "game_instance", "game instance exited with status: {}", exit_status.to_string());
                    return Err(instance::instance::Error::ProcessError { reason: exit_status.to_string() });
                }
            },
            Err(err) => {
                tracing::error!(name: "game_instance", "game instance exited with error: {:?}", err);
                return Err(err)
            },
        }
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
