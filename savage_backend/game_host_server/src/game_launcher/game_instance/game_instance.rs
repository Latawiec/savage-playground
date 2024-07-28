use tokio::io::AsyncReadExt;
use tracing::error;

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
            .take_stderr();

        // TODO: Fix this to properly display errors.
        tokio::spawn(async {
            if let Some(mut stderr) = stderr {
                let mut output = String::new();
                while let Ok(size) = stderr.read_to_string(&mut output).await {
                    println!("Process error: {}", output);
                }
            }
        });

        Ok(GameInstance {
            _instance: instance,
            stdin,
            stdout,
            stderr: None,
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
