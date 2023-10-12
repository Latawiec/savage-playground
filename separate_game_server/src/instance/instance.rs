use std::{process::{Stdio, ExitStatus}, io::{Read, Write}};
use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub enum Error {
    StartupError { reason: String },
    PipeError { reason: String },
    ProcessError { reason: String },
}

pub struct Instance {
    path: String,
    process: Child,
}

impl Drop for Instance {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

impl Instance {
    pub fn new(path: String) -> Result<Instance, Error> {
        let mut process = match Command::new(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
                Err(error) => {
                    return Err(Error::StartupError { reason: error.to_string() })
                },
                Ok (process) => process,
            };

        Ok(Instance {
            path,
            process,
        })
    }

    pub fn split_io(&mut self) -> (Option<ChildStdin>, Option<ChildStdout>) {
        (self.process.stdin.take(), self.process.stdout.take())
    }

    pub fn try_wait(&mut self) -> Result<Option<ExitStatus>, Error> {
        self.process.try_wait().map_err(|err| {
            Error::ProcessError { reason: err.to_string() }
        })
    }
}