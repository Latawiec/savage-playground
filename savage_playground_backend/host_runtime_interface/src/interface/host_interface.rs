use async_trait::async_trait;

use crate::proto::{host_client, host_instance};

#[async_trait]
pub trait HostInstanceInterface {
    async fn send(&self, msg: &host_instance::ClientMessage);
    async fn read(&self) -> Option<host_instance::InstanceMessage>;
}

#[async_trait]
pub trait HostClientInterface {
    async fn send(&self, msg: &host_client::HostMessage);
    async fn read(&self) -> Option<host_client::ClientMessage>;
}
