use std::time::Duration;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
    time,
};

pub struct TimeSlicedIO {
    stdin_channel: mpsc::Receiver<Vec<u8>>,
    stdout_channel: mpsc::Sender<Vec<u8>>,
    stderr_channel: mpsc::Sender<Vec<u8>>,
    rt: tokio::runtime::Runtime,
}

impl TimeSlicedIO {
    const STDIN_CHANNEL_CAPACITY: usize = 16;
    const STDOUT_CHANNEL_CAPACITY: usize = 16;
    const STDERR_CHANNEL_CAPACITY: usize = 16;

    pub fn stdout(&mut self, data: Vec<u8>) {
        if let Err(error) = self.stdout_channel.blocking_send(data) {
            tracing::error!("Failed to write to stdout: {}", error);
        }
    }

    pub fn stderr(&mut self, data: Vec<u8>) {
        if let Err(error) = self.stderr_channel.blocking_send(data) {
            tracing::error!("Failed to write to stdout: {}", error);
        }
    }

    pub fn stdin(&mut self) -> Option<Vec<u8>> {
        match self.stdin_channel.try_recv() {
            Ok(data) => Some(data),
            Err(e) => match e {
                mpsc::error::TryRecvError::Empty => None,
                mpsc::error::TryRecvError::Disconnected => {
                    tracing::error!("Channel for stdin has disconnected.");
                    panic!("Channel for stdin has disconnected.");
                },
            },
        }
    }

    pub fn run_for(&self, duration: Duration) {
        let _guard = &self.rt.enter();
        let _ = &self.rt.block_on(time::sleep(duration));
    }

    async fn task_stdout(mut channel: mpsc::Receiver<Vec<u8>>) {
        let mut stdout = io::stdout();
        while let Some(msg) = channel.recv().await {
            let data_len = msg.len();
            if let Err(error) = stdout.write_u64(data_len as u64).await {
                tracing::error!("Failed to send message length to stdout: {}", error);
            }
            if let Err(error) = stdout.write_all(&msg).await {
                tracing::error!("Failed to send message data to stdout: {}", error);
            }
        }
        if let Err(error) = stdout.flush().await {
            tracing::error!("Failed to flush stdout: {}", error);
        }
    }

    async fn task_stderr(mut channel: mpsc::Receiver<Vec<u8>>) {
        let mut stderr = io::stderr();
        while let Some(msg) = channel.recv().await {
            let data_len = msg.len();
            if let Err(error) = stderr.write_u64(data_len as u64).await {
                tracing::error!("Failed to send message length to stderr: {}", error);
            }
            if let Err(error) = stderr.write_all(&msg).await {
                tracing::error!("Failed to send message data to stderr: {}", error);
            }
        }
        if let Err(error) = stderr.flush().await {
            tracing::error!("Failed to flush stderr: {}", error);
        }
    }

    async fn task_stdin(channel: mpsc::Sender<Vec<u8>>) {
        let mut stdin: io::Stdin = io::stdin();
        let mut buffer: Vec<u8> = vec![0; 512];
        loop {
            let data_len = stdin.read_u64().await.unwrap();
            buffer.resize(data_len as usize, 0);
            if let Err(error) = stdin.read_exact(&mut buffer).await {
                tracing::error!("Failed to read from stdin: {}", error);
            }
            if let Err(error) = channel.send(buffer.clone()).await {
                tracing::error!("Failed to send stdin message to the channel: {}", error);
            }
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

        let _guard = rt.enter();
        rt.spawn(TimeSlicedIO::task_stdin(stdin_sender));
        rt.spawn(TimeSlicedIO::task_stdout(stdout_receiver));
        rt.spawn(TimeSlicedIO::task_stderr(stderr_receiver));

        Self {
            stdin_channel: stdin_receiver,
            stdout_channel: stdout_sender,
            stderr_channel: stderr_sender,
            rt,
        }
    }
}