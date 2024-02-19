use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use rocket::{delete, FromForm, State};
use rocket::{get, put};
use rocket::serde::json::Json;

use crate::game_launcher;
use crate::game_launcher::game_launcher::GameLauncher;
use crate::server::connection::client_connection::ClientConnection;
use crate::server::game_host;
use crate::server::game_host::game_host::GameHost;
use room_server_interface::schema::game_config::GameConfig;

use super::error::APIError;
use super::types::RoomsData;

type RoomId = u64;

struct TestState {
    val: String,
}

#[get("/create_room?<game_room_config..>")]
pub fn create_room(
    remote_addr: std::net::SocketAddr,
    ws: rocket_ws::WebSocket,
    game_room_config: Json<GameConfig>,
    game_launcher: &State<Arc<GameLauncher>>,
    game_host: &State<Arc<GameHost>>,
) -> Result<rocket_ws::Channel<'static>, APIError> {
    let game_host = game_host.inner().clone();
    let room_handle = match game_host.create_room(game_room_config.0, game_launcher) {
        Some(ok) => ok,
        None => return Err(APIError::Bad("Couldn't create the room".to_owned())),
    };

    Ok(
        ws.channel(move |stream| {
            Box::pin(async move{                
                let mut connection_handle = match game_host.join_room(room_handle, stream) {
                    Some(ok) => ok,
                    None => return Err(rocket_ws::result::Error::ConnectionClosed),
                };

                let result = connection_handle.wait().await;
                println!("Disconnected: {:?}", result);
                Ok(())
            })
        })
    )
}

#[put("/update_room/<room_id>", data = "<room_data>")]
pub fn update_room(remote_addr: std::net::SocketAddr, room_id: RoomId, room_data: Json<GameConfig>) -> &'static str {
    "woop woop"
}

#[get("/join_room/<room_id>")]
pub fn join_room(remote_addr: std::net::SocketAddr, ws: rocket_ws::WebSocket, room_id: RoomId) -> &'static str {
    "oof."
}

#[delete("/destroy_room/<room_id>")]
pub fn destroy_room(room_id: RoomId) -> () {
    ()
}

#[get("/rooms_data")]
pub fn get_rooms() -> Json<RoomsData> {
    Json(RoomsData{
        helo: true,
    })
}
