use bevy::prelude::Resource;
use datazoo::JaggedVec;
use std::{
    io::{BufRead, Write},
    sync::{mpsc, Arc, RwLock},
    thread::JoinHandle,
};

use super::IOInterface;

// Since I couldn't find any non-blocking way to do this except tokio, and bevy is entirely non-async
// I'm gonna have to implement this with a thread for stdin.
pub struct UnnamedPipesGameIO {
    // TODO: Make shared buffer for this.
    stdin_buffer: Arc<RwLock<JaggedVec<u8>>>,
    stdout_buffer: Vec<u8>,
    _reader_handle: JoinHandle<()>,
}

impl Default for UnnamedPipesGameIO {
    fn default() -> Self {
        let (sender, _) = mpsc::channel::<()>();
        let stdin_buffer: Arc<RwLock<JaggedVec<u8>>> = Default::default();

        let stdin_buffer_clone = stdin_buffer.clone();
        let _reader_handle = std::thread::spawn(move || {
            let mut temp_buff = Vec::<u8>::new();
            loop {
                std::io::stdin().lock().read_until(b'\0', &mut temp_buff);
                stdin_buffer_clone
                    .write()
                    .unwrap()
                    .push_row(temp_buff[0..&temp_buff.len() - 1].iter().cloned()); // Cut the '\0'
                sender.send(());
                temp_buff.clear();
            }
        });

        Self {
            stdin_buffer,
            stdout_buffer: Default::default(),
            _reader_handle,
        }
    }
}

impl IOInterface for UnnamedPipesGameIO {
    fn write_msg(&mut self, data: &[u8]) {
        std::io::stdout().lock().write_all(data);
    }

    fn read_msg(&mut self) -> Option<&[u8]> {
        self.stdout_buffer.clear();
        let mut buffer_lock = self.stdin_buffer.write().unwrap();
        if let Some(msg) = buffer_lock.pop_row() {
            self.stdout_buffer.extend_from_slice(&*msg);
            return Some(&self.stdout_buffer);
        }
        None
    }
}
