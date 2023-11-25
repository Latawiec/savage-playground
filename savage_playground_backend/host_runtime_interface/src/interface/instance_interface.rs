use crate::proto::host_instance::{ClientMessage, InstanceMessage};

pub trait InstanceInterface {
    fn send(msg: &InstanceMessage);
    fn read() -> Option<ClientMessage>;
}
