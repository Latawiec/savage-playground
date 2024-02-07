use std::{
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};

#[derive(Debug)]
pub enum Error {
    StartupError { reason: String },
    ProcessError { reason: String },
    KillError { reason: String },
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::StartupError { reason } => reason.to_owned(),
            Error::ProcessError { reason } => reason.to_owned(),
            Error::KillError { reason } => reason.to_owned(),
        }
    }
}

pub struct Instance {
    cwd_path: PathBuf,
    exe_path: PathBuf,
    process: Child,
}

impl Drop for Instance {
    fn drop(&mut self) {
        // We can't do anything better here - Drop is sync operation.
        let _ = self.process.start_kill();
    }
}

impl Instance {
    pub fn new(cwd_path: &Path, exe_path: &Path) -> Result<Instance, Error> {
        let process = match Command::new(&exe_path)
            .current_dir(&cwd_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Err(error) => {
                return Err(Error::StartupError {
                    reason: error.to_string(),
                })
            }
            Ok(process) => process,
        };

        Ok(Instance {
            cwd_path: cwd_path.to_owned(),
            exe_path: exe_path.to_owned(),
            process,
        })
    }

    pub fn take_stdin(&mut self) -> Option<ChildStdin> {
        self.process.stdin.take()
    }

    pub fn take_stdout(&mut self) -> Option<ChildStdout> {
        self.process.stdout.take()
    }

    pub fn take_stderr(&mut self) -> Option<ChildStderr> {
        self.process.stderr.take()
    }

    pub fn try_wait(&mut self) -> Result<Option<ExitStatus>, Error> {
        self.process.try_wait().map_err(|err| Error::ProcessError {
            reason: err.to_string(),
        })
    }

    pub async fn wait(&mut self) -> Result<ExitStatus, Error> {
        self.process.wait().await.map_err(|err| Error::ProcessError { reason: err.to_string() })
    }

    pub async fn kill(&mut self) -> Result<ExitStatus, Error> {
        if let Err(error) = self.process.start_kill() {
            return Err(Error::KillError{ reason: error.to_string() })
        }
        self.wait().await
    }
}
