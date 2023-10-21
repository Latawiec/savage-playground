

use std::{path::PathBuf, str::FromStr};

use asset_server::server::AssetServerHandle;
use instance::instance::Instance;
use room_server::{server::RoomServerHandle, message::Message};
use tracing_subscriber::fmt::format::FmtSpan;
use tokio::{io::AsyncReadExt, io::AsyncWriteExt};


mod config;
mod asset_server;
mod room_server;
mod instance;
mod game_host;

// Move it
pub fn str_from_u8_nul_utf8(utf8_src: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let nul_range_end = utf8_src.iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    ::std::str::from_utf8(&utf8_src[0..nul_range_end])
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();
    
    let (room_server, room_server_handle) = RoomServerHandle::new(([127, 0, 0, 1], 8080).into());
    let (asset_Server, asset_server_handle) = AssetServerHandle::new(([127, 0, 0, 1], 8081).into());

    tokio::spawn(async move {
        let mut receiver = room_server.subscribe();
        while let Ok(notification) = receiver.recv().await {
            match notification {
                room_server::server::RoomServerNotification::RoomCreated { room_id, client_id: _ } => {
                    if let Some((mut receiver, sender)) = room_server.get_room_channels(room_id) {
                        let game_dir = PathBuf::from_str("/mnt/b981039f-fbe8-4a78-be47-2fd24cb3be26/Programing/RustTesting/savage_playground/target/debug/debug").unwrap();
                        let mut process = Instance::new(&game_dir).unwrap();
                        let (mut stdin, mut stdout, mut _stderr) = (process.take_stdin().unwrap(), process.take_stdout().unwrap(), process.take_stderr().unwrap());

                        tracing::info!("Room {} created.", room_id);

                        tokio::spawn(async move {
                            let mut buffer = vec![0; 4096];

                            loop {
                                if let Ok(bytes_read) = stdout.read(&mut buffer).await {
                                    if bytes_read != 0 {
                                        let _ = sender.send(room_server::message::ServerMessage::Room { room_id, message: Message::Text{ data: str_from_u8_nul_utf8(&buffer).unwrap().to_owned() } });
                                        buffer[0..bytes_read].fill(0);
                                    }
                                } else {
                                    tracing::error!("Read from process failed");
                                    break;
                                }
                            }
                        });

                        tokio::spawn(async move {
                            while let Ok(msg) = receiver.recv().await {
                                match msg {
                                        room_server::message::ClientMessage::Connected { client_id } => tracing::info!("{} connected!", client_id),
                                        room_server::message::ClientMessage::Disconnected { client_id } => tracing::info!("{} disconnected!", client_id),
                                        room_server::message::ClientMessage::JoinedRoom { client_id, room_id } => tracing::info!("{} joined room {}!", client_id, room_id),
                                        room_server::message::ClientMessage::LeftRoom { client_id, room_id } => tracing::info!("{} left room {}!", client_id, room_id),
                                        room_server::message::ClientMessage::Data { client_id, message } => {
                                            tracing::info!("{} says: {:?}", client_id, message);
                                            let mut msg = match message {
                                                Message::Text { data } => data.as_bytes().to_owned(),
                                                Message::Binary { data } => data,
                                            };
                                            msg.push(b'\n'); // TODO avoid this. Program on the other side should not be waiting for endline.
                                            match stdin.write_all(&msg).await {
                                                Ok(_) => {
                                                    tracing::info!("Sent: {:?}", msg);
                                                },
                                                Err(e) => {
                                                    tracing::error!("Send to process failed: {:?}", e);
                                                },
                                            }
                                        }
                                };
                            }
                        });
                    }
                },
                room_server::server::RoomServerNotification::RoomClosed { room_id: _ } => {
                    
                },
                room_server::server::RoomServerNotification::RoomEmpty { room_id } => {
                    tracing::info!("Room {} is empty. Deleting", room_id);
                    room_server.close_room(room_id);
                }
            }
        }
    });
    let _ = tokio::join!(room_server_handle, asset_server_handle);
}