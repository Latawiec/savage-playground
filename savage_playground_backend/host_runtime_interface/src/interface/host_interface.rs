use crate::proto::{host_client, host_instance};

pub trait HostInstanceInterface {
    fn send(msg: &host_instance::ClientMessage);
    fn read() -> Option<host_instance::InstanceMessage>;
}

pub trait HostClientInterface {
    fn send(msg: &host_client::HostMessage);
    fn read() -> Option<host_client::ClientMessage>;
}
