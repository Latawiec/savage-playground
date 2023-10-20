use std::net::SocketAddr;
use std::hash::Hash;

use tokio::task::JoinHandle;


pub type ClientID = u64;

pub (crate) struct Client {
    pub id: ClientID,
    pub addr: Option<SocketAddr>,
    pub (super) message_read_task_handle: JoinHandle<()>,
    pub (super) message_write_task_handle: JoinHandle<()>,
}

impl Hash for Client {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.message_write_task_handle.abort();
        self.message_read_task_handle.abort();
    }
}