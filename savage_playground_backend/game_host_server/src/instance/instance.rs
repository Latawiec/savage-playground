use std::{process::{Stdio, ExitStatus}, path::{Path, PathBuf}};
use tokio::process::{Command, Child, ChildStdin, ChildStdout, ChildStderr};

#[derive(Debug)]
pub enum Error {
    StartupError { reason: String },
    ProcessError { reason: String },
}

pub struct Instance {
    path: PathBuf,
    process: Child,
}

impl Drop for Instance {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

impl Instance {
    pub fn new(path: &Path) -> Result<Instance, Error> {
        let process = match Command::new(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
                Err(error) => {
                    return Err(Error::StartupError { reason: error.to_string() })
                },
                Ok (process) => process,
            };

        Ok(Instance {
            path: path.to_owned(),
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
        self.process.try_wait().map_err(|err| {
            Error::ProcessError { reason: err.to_string() }
        })
    }
}