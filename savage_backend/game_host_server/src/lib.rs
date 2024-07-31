mod api;
mod config;
mod game_host;
mod game_launcher;
mod instance;
mod util;

use rocket::http::Method;
use rocket::{routes, Config};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::path::PathBuf;
use std::sync::Arc;

use crate::game_host::game_host::GameHost;
use crate::game_launcher::game_launcher::GameLauncher;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
}

pub struct GameHostServerConfig {
    pub server_base_path: String,
    pub port: Option<u16>,
    pub games_root_directory: PathBuf,
    pub games_map_file: PathBuf,
    pub allowed_origins_regex: Vec<&'static str>
}

pub struct GameHostServer {
    server_base_path: String,
    pub port: Option<u16>,
    game_launcher: Arc<GameLauncher>,
    game_host: Arc<GameHost>,
    allowed_origins: AllowedOrigins
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

        let server_base_path = config.server_base_path;
        let port = config.port;
        let game_launcher = Arc::new(game_launcher.unwrap());
        let game_host = Arc::new(GameHost::new());
        let allowed_origins = AllowedOrigins::some_regex(&config.allowed_origins_regex);

        Ok(GameHostServer {
            server_base_path,
            port,
            game_launcher,
            game_host,
            allowed_origins,
        })
    }

    pub async fn serve(&self) -> Result<(), Error> {
        let cors = CorsOptions {
            allowed_origins: self.allowed_origins.clone(),
            allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
            allowed_headers: AllowedHeaders::some(&[
                "Access-Control-Allow-Origin",
                "Content-Type"
            ]),
            ..Default::default()
        }
        .to_cors();
        if let Err(err) = cors {
            return Err(Error::Unknown(err.to_string()));
        }

        let mut config = Config::default();
        if let Some(port) = self.port {
            config.port = port;
        }
        
        rocket::custom(config)
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
            .attach(cors.unwrap())
            .ignite().await.map_err(|e| Error::Unknown(format!("Ignition error: {}", e)))?
            .launch().await.map_err(|e| Error::Unknown(format!("Launch error: {}", e)))?;

        Ok(())
    }
}
