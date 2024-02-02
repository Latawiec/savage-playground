use super::{
    traits::room_host_communication::{RoomHostInput, RoomHostOutput},
    types::{
        error::RoomHostError, room_client_message::RoomClientMessage,
        room_host_message::RoomHostMessage,
    },
};

pub struct RoomHostMessenger {
    client_message_sender: tokio::sync::broadcast::Sender<RoomClientMessage>,
    host_message_sender: tokio::sync::broadcast::Sender<RoomHostMessage>,
}

impl Default for RoomHostMessenger {
    fn default() -> Self {
        const DEFAULT_BROADCAST_QUEUE_SIZE: usize = 1024;
        let (client_message_sender, _) = tokio::sync::broadcast::channel(DEFAULT_BROADCAST_QUEUE_SIZE);
        let (host_message_sender, _) = tokio::sync::broadcast::channel(DEFAULT_BROADCAST_QUEUE_SIZE);
        Self { client_message_sender, host_message_sender }
    }
}

impl RoomHostInput for RoomHostMessenger {
    async fn send_client_msg(
        &self,
        client_message: RoomClientMessage,
    ) -> Result<(), RoomHostError> {
        if let Err(e) = self.client_message_sender.send(client_message) {
            return Err(RoomHostError::InternalError {
                message: format!("Error sending Client Message: {}", e.to_string()),
            });
        }
        Ok(())
    }

    fn subscribe_client_msg(
        &self,
    ) -> Result<tokio::sync::broadcast::Receiver<RoomClientMessage>, RoomHostError> {
        Ok(self.client_message_sender.subscribe())
    }
}

impl RoomHostOutput for RoomHostMessenger {
    async fn send_host_msg(&self, host_message: RoomHostMessage) -> Result<(), RoomHostError> {
        if let Err(e) = self.host_message_sender.send(host_message) {
            return Err(RoomHostError::InternalError {
                message: format!("Error sending Host Message: {}", e.to_string()),
            });
        }
        Ok(())
    }

    fn subscribe_host_msg(
        &self,
    ) -> Result<tokio::sync::broadcast::Receiver<RoomHostMessage>, RoomHostError> {
        Ok(self.host_message_sender.subscribe())
    }
}
