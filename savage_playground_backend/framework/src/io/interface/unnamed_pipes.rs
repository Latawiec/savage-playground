use bevy::prelude::Resource;
use std::{
    io::{BufRead, Write},
    sync::{mpsc, Arc, RwLock},
    thread::JoinHandle,
};

use super::{IOInterface, PushVec};

// Since I couldn't find any non-blocking way to do this except tokio, and bevy is entirely non-async
// I'm gonna have to implement this with a thread for stdin.
pub struct UnnamedPipesGameIO {
    // TODO: Make shared buffer for this.
    stdin_buffer: Arc<RwLock<PushVec<u8>>>,
    _reader_handle: JoinHandle<()>,
}

impl Default for UnnamedPipesGameIO {
    fn default() -> Self {
        let (sender, _) = mpsc::channel::<()>();
        let stdin_buffer: Arc<RwLock<PushVec<u8>>> = Default::default();

        let stdin_buffer_clone = stdin_buffer.clone();
        let _reader_handle = std::thread::spawn(move || {
            let mut temp_buff = Vec::<u8>::new();
            loop {
                std::io::stdin().lock().read_until(b'\0', &mut temp_buff);
                stdin_buffer_clone.write().unwrap().push(&temp_buff);
                sender.send(());
                temp_buff.clear();
            }
        });

        Self {
            stdin_buffer,
            _reader_handle,
        }
    }
}

impl IOInterface for UnnamedPipesGameIO {
    fn write(&mut self, data: &[u8]) {
        std::io::stdout().lock().write_all(data);
    }

    fn read(&mut self, buffer: &mut PushVec<u8>) {
        let mut buffer_lock = self.stdin_buffer.write().unwrap();
        for message in buffer_lock.iter() {
            buffer.push(message);
        }
        buffer_lock.clear();
    }
}
