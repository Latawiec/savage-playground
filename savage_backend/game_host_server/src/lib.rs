mod api;
mod config;
mod game_host;
mod game_launcher;
mod instance;

use rocket::routes;
use std::path::PathBuf;
use std::sync::Arc;

use crate::game_host::game_host::GameHost;
use crate::game_launcher::game_launcher::GameLauncher;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub struct GameHostServerConfig {
    port: Option<u16>,
    name: Option<String>,
    server_base_path: Option<String>,
    games_root_directory: PathBuf,
    games_map_file: PathBuf,
}

pub struct GameHostServer {
    server_base_path: String,
    game_launcher: Arc<GameLauncher>,
    game_host: Arc<GameHost>,
}

impl GameHostServer {
    pub fn new(config: GameHostServerConfig) -> Result<GameHostServer, Error> {
        let game_launcher = GameLauncher::new(&config.games_root_directory, &config.games_map_file);
        if let Err(err) = game_launcher {
            return Err(Error::Unknown(format!(
                "Couldn't initialize GameLauncher: {:?}",
                err
            )));
        }
        let game_launcher = Arc::new(game_launcher.unwrap());
        let game_host = Arc::new(GameHost::new());

        Ok(GameHostServer {
            game_launcher,
            game_host,
            server_base_path: config.server_base_path.unwrap_or_default(),
        })
    }

    pub async fn serve(&self) -> Result<(), Error> {
        rocket::build()
            .manage(self.game_launcher.clone())
            .manage(self.game_host.clone())
            .mount(
                self.server_base_path.clone(),
                routes![
                    api::api::create_room,
                    api::api::get_rooms,
                    api::api::join_room,
                    api::api::destroy_room
                ],
            )
            .ignite().await.map_err(|e| Error::Unknown(format!("Ignition error: {}", e)))?
            .launch().await.map_err(|e| Error::Unknown(format!("Launch error: {}", e)))?;

        Ok(())
    }
}
