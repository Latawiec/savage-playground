use async_trait::async_trait;

use crate::proto::host_instance::{ClientMessage, InstanceMessage};

#[async_trait]
pub trait InstanceInterface {
    async fn send(&self, msg: &InstanceMessage);
    async fn read(&self) -> Option<ClientMessage>;
}
