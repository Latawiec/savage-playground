use std::{path::PathBuf, sync::Arc};

use game_host_server::{
    game_launcher::game_launcher::GameLauncher, server::game_host::game_host::GameHost,
};

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    let dir_path = PathBuf::from("/");
    let file_path = PathBuf::from("./src/test_game_mapping.json");
    rocket::build()
        .manage(Arc::new(GameLauncher::new(&dir_path, &file_path).unwrap()))
        .manage(Arc::new(GameHost::new()))
        .mount(
            "/",
            routes![
                game_host_server::server::api::api::create_room,
                game_host_server::server::api::api::get_rooms,
                game_host_server::server::api::api::join_room,
                game_host_server::server::api::api::destroy_room,
            ],
        )
}
