use std::sync::Arc;

use rocket::serde::json::Json;
use rocket::{delete, get, State};

use crate::game_launcher::game_launcher::GameLauncher;
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
    game_host_state: &State<Arc<GameHost>>,
) -> Result<rocket_ws::Channel<'static>, APIError> {
    let game_host = game_host_state.inner().clone();
    let room_handle = match game_host.create_room(game_room_config.0, game_launcher) {
        Some(ok) => ok,
        None => return Err(APIError::Bad("Couldn't create the room".to_owned())),
    };

    join_room(remote_addr, ws, room_handle.0, game_host_state)
}

#[get("/join_room/<room_id>")]
pub fn join_room(
    remote_addr: std::net::SocketAddr,
    ws: rocket_ws::WebSocket,
    room_id: u64,
    game_host: &State<Arc<GameHost>>,
) -> Result<rocket_ws::Channel<'static>, APIError> {
    println!("Huh?");
    let game_host = game_host.inner().clone();

    Ok(ws.channel(move |stream| {
        Box::pin(async move {
            match game_host.join_room(room_id.into(), stream).await {
                game_host::disconnect_reason::DisconnectReason::ClientDisconnected => todo!(),
                game_host::disconnect_reason::DisconnectReason::ClientClosedConnection => todo!(),
                game_host::disconnect_reason::DisconnectReason::ClientConnectionDestroyed => {
                    todo!()
                }
                game_host::disconnect_reason::DisconnectReason::RoomClosed => todo!(),
                game_host::disconnect_reason::DisconnectReason::RoomDoesNotExist => todo!(),
                game_host::disconnect_reason::DisconnectReason::UnexpectedError(_) => todo!(),
                game_host::disconnect_reason::DisconnectReason::GameCrashed => todo!(),
            }
        })
    }))
}

#[delete("/destroy_room/<room_id>")]
pub fn destroy_room(room_id: u64, game_host: &State<Arc<GameHost>>) -> () {
    game_host.delete_room(room_id.into());
}

#[get("/rooms_data")]
pub fn get_rooms() -> Json<RoomsData> {
    Json(RoomsData { helo: true })
}
