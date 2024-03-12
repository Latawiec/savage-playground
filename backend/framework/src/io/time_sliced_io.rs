use std::{sync::Arc, time::Duration};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
    task::LocalSet,
    time,
};

pub struct TimeSlicedIO {
    stdin_channel: mpsc::Receiver<Vec<u8>>,
    stdout_channel: mpsc::Sender<Vec<u8>>,
    stderr_channel: mpsc::Sender<Vec<u8>>,
    workpool: LocalSet,
    rt: tokio::runtime::Runtime,
}

impl TimeSlicedIO {
    const STDIN_CHANNEL_CAPACITY: usize = 16;
    const STDOUT_CHANNEL_CAPACITY: usize = 16;
    const STDERR_CHANNEL_CAPACITY: usize = 16;

    pub fn stdout(&mut self, data: Vec<u8>) {
        self.stdout_channel.send(data);
    }

    pub fn stderr(&mut self, data: Vec<u8>) {
        self.stderr_channel.send(data);
    }

    pub fn stdin(&mut self) -> Option<Vec<u8>> {
        match self.stdin_channel.try_recv() {
            Ok(data) => Some(data),
            Err(e) => match e {
                mpsc::error::TryRecvError::Empty => None,
                mpsc::error::TryRecvError::Disconnected => panic!("Channel died."),
            },
        }
    }

    pub fn run_for(&self, duration: Duration) {
        self.workpool.block_on(&self.rt, time::sleep(duration));
    }

    async fn task_stdout(mut channel: mpsc::Receiver<Vec<u8>>) {
        let mut stdout = io::stdout();
        while let Some(msg) = channel.recv().await {
            let message_size_data = msg.len().to_ne_bytes();
            stdout.write_all(&message_size_data).await;
            stdout.write_all(&msg).await;
        }
    }

    async fn task_stderr(mut channel: mpsc::Receiver<Vec<u8>>) {
        let mut stderr = io::stderr();
        while let Some(msg) = channel.recv().await {
            let message_size_data = msg.len().to_ne_bytes();
            stderr.write_all(&message_size_data).await;
            stderr.write_all(&msg).await;
        }
    }

    async fn task_stdin(channel: mpsc::Sender<Vec<u8>>) {
        let mut stdin: io::Stdin = io::stdin();
        let mut buffer: Vec<u8> = vec![0; 512];
        loop {
            let message_size = stdin.read_u64().await.unwrap();
            buffer.resize(message_size as usize, 0);
            stdin.read_exact(&mut buffer).await;
            channel.send(buffer.clone()).await;
        }
    }
}

impl Default for TimeSlicedIO {
    fn default() -> Self {
        let (stdin_sender, stdin_receiver) = mpsc::channel(TimeSlicedIO::STDIN_CHANNEL_CAPACITY);
        let (stdout_sender, stdout_receiver) = mpsc::channel(TimeSlicedIO::STDOUT_CHANNEL_CAPACITY);
        let (stderr_sender, stderr_receiver) = mpsc::channel(TimeSlicedIO::STDERR_CHANNEL_CAPACITY);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let workpool = LocalSet::new();

        workpool.spawn_local(TimeSlicedIO::task_stdin(stdin_sender));
        workpool.spawn_local(TimeSlicedIO::task_stdout(stdout_receiver));
        workpool.spawn_local(TimeSlicedIO::task_stderr(stderr_receiver));

        Self {
            stdin_channel: stdin_receiver,
            stdout_channel: stdout_sender,
            stderr_channel: stderr_sender,
            workpool,
            rt,
        }
    }
}
