use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{ChildStderr, ChildStdin, ChildStdout},
};

pub struct ProtoStdin {
    stdin: ChildStdin,
}

impl ProtoStdin {
    pub fn new(stdin: ChildStdin) -> ProtoStdin {
        ProtoStdin { stdin }
    }

    pub async fn send<T: prost::Message>(&mut self, msg: &T) {
        let mut data = Vec::<u8>::new();
        
        if let Err(err) = msg.encode(&mut data) {
            tracing::error!(name: "proto_pipe", pipe = "stdin", "couldn't encode message: {}", err);
            return;
        }
        let data_len = data.len();

        if let Err(err) = self.stdin.write_u64(data_len as u64).await {
            tracing::error!(name: "proto_pipe", pipe = "stdin", "couldn't write message length: {}", err);
            return;
        }
        if let Err(err) = self.stdin.write_all(&data).await {
            tracing::error!(name: "proto_pipe", pipe = "stdin", "couldn't write message content: {}", err);
            return;
        }
        if let Err(err) = self.stdin.flush().await {
            tracing::error!(name: "proto_pipe", pipe = "stdin", "couldn't flush buffer: {}", err);
            return;
        }
        tracing::trace!(name: "proto_pipe", pipe = "stdin", data_len, "stdin sent");
    }

    pub async fn send_many<T: prost::Message>(&mut self, msgs: &[T]) {
        let mut data = Vec::<u8>::new();
        let mut total_data_len = 0;
        for msg in msgs {
            if let Err(err) = msg.encode(&mut data) {
                tracing::error!(name: "proto_pipe", pipe = "stdin", "couldn't encode message: {}", err);
                return;
            }
            let data_len = data.len();

            if let Err(err) = self.stdin.write_u64(data_len as u64).await {
                tracing::error!(name: "proto_pipe", pipe = "stdin", "couldn't write message length: {}", err);
                return;
            }
            if let Err(err) = self.stdin.flush().await {
                tracing::error!(name: "proto_pipe", pipe = "stdin", "couldn't flush buffer: {}", err);
                return;
            }
            total_data_len += data_len;
        }
        if let Err(err) = self.stdin.flush().await {
            tracing::error!(name: "proto_pipe", pipe = "stdin", "couldn't flush buffer: {}", err);
            return;
        }
        tracing::trace!(name: "proto_pipe", pipe = "stdin", total_data_len, "stdin sent many");
    }
}

pub struct ProtoStdout {
    stdout: BufReader<ChildStdout>,
}

impl ProtoStdout {
    pub fn new(stdout: ChildStdout) -> ProtoStdout {
        ProtoStdout {
            stdout: BufReader::new(stdout),
        }
    }

    pub async fn read<T: prost::Message + Default>(&mut self) -> Option<T> {
        let data_len = self.stdout.read_u64().await;
        if let Err(err) = data_len {
            tracing::error!(name: "proto_pipe", pipe = "stdout", "couldn't read message length: {}", err);
            return None;
        };
        let data_len = data_len.unwrap();

        tracing::trace!(name: "proto_pipe", pipe = "stdout", data_len, "stdout read");
        let mut data = Vec::<u8>::new();
        data.resize(data_len as usize, 0);

        if let Err(err) = self.stdout.read_exact(&mut data).await {
            tracing::error!(name: "proto_pipe", pipe = "stdout", "couldn't read expected message length: {}", err);
            return None;
        }

        match T::decode(data.as_slice()) {
            Ok(message) => {
                return Some(message);
            },
            Err(err) => {
                tracing::error!(name: "proto_pipe", pipe = "stdout", "couldn't dencode message: {}", err);
                return None;
            }
        }
    }
}

pub struct ProtoStderr {
    stderr: BufReader<ChildStderr>,
}

impl ProtoStderr {
    pub fn new(stderr: ChildStderr) -> ProtoStderr {
        ProtoStderr {
            stderr: BufReader::new(stderr),
        }
    }

    pub async fn read<T: prost::Message + Default>(&mut self) -> Option<T> {
        let data_len = self.stderr.read_u64().await;
        if let Err(err) = data_len {
            tracing::error!(name: "proto_pipe", pipe = "stderr", "couldn't read message length: {}", err);
            return None;
        };
        let data_len = data_len.unwrap();

        tracing::trace!(name: "proto_pipe", pipe = "stderr", data_len, "stderr read");
        let mut data = Vec::<u8>::new();
        data.resize(data_len as usize, 0);

        if let Err(err) = self.stderr.read_exact(&mut data).await {
            tracing::error!(name: "proto_pipe", pipe = "stderr", "couldn't read expected message length: {}", err);
            return None;
        }

        match T::decode(data.as_slice()) {
            Ok(message) => {
                return Some(message);
            },
            Err(err) => {
                tracing::error!(name: "proto_pipe", pipe = "stderr", "couldn't dencode message: {}", err);
                return None;
            }
        }
    }

    pub fn raw_reader(&mut self) -> &mut impl AsyncRead {
        &mut self.stderr
    }
}
