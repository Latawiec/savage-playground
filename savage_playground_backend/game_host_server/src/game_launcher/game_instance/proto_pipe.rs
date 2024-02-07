use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
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
        msg.encode(&mut data).unwrap();
        let data_len = data.len();
        _ = self.stdin.write_u64(data_len as u64).await;
        _ = self.stdin.write(&data).await;
        _ = self.stdin.flush().await;
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
        let data_len = self.stdout.read_u64().await.unwrap();
        let mut data = Vec::<u8>::new();
        data.resize(data_len as usize, 0);
        _ = self.stdout.read_exact(&mut data).await;

        let message = T::decode(data.as_slice());
        return Some(message.unwrap());
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
        let data_len = self.stderr.read_u64().await.unwrap();
        let mut data = Vec::<u8>::new();
        data.resize(data_len as usize, 0);
        _ = self.stderr.read_exact(&mut data).await;

        let message = T::decode(data.as_slice());
        return Some(message.unwrap());
    }
}
