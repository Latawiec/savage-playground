use std::time::Duration;

use server::server::ServerHandle;
use tracing_subscriber::fmt::format::FmtSpan;
use tokio::runtime::{Runtime, Handle};


mod config;
mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();
    
    let (server, handle) = ServerHandle::new(([127, 0, 0, 1], 8080).into());

    tokio::spawn(async move {
        let mut receiver = server.subscribe();
        while let Ok(notification) = receiver.recv().await {
            match notification {
                server::server::ServerNotification::RoomCreated { room_id } => {
                    if let Some((mut receiver, sender)) = server.get_room_channels(room_id).await {
                        tokio::spawn(async move {
                            loop {
                                let _ = sender.send(server::message::ServerMessage::Broadcast {  });
                                tokio::time::sleep(Duration::from_secs(5)).await;
                            }
                        });

                        tokio::spawn(async move {
                            while let Ok(msg) = receiver.recv().await {
                                match msg {
                                        server::message::ClientMessage::Connected { client_id } => tracing::info!("{} connected!", client_id),
                                        server::message::ClientMessage::Disconnected { client_id } => tracing::info!("{} disconnected!", client_id),
                                        server::message::ClientMessage::JoinedRoom { client_id, room_id } => tracing::info!("{} joined room {}!", client_id, room_id),
                                        server::message::ClientMessage::LeftRoom { client_id, room_id } => tracing::info!("{} left room {}!", client_id, room_id),
                                        server::message::ClientMessage::String { client_id, text } => tracing::info!("{} says: {}", client_id, text),
                                        server::message::ClientMessage::Binary { client_id, data } => todo!(),
                                };
                            }
                        });
                    }
                },
                server::server::ServerNotification::RoomClosed { room_id } => {
                    
                },
                server::server::ServerNotification::RoomEmpty { room_id } => {
                    tracing::info!("Room {} is empty. Deleting", room_id);
                    server.close_room(room_id).await;
                }
            }
        }
    });

    let _ = handle.await;
}