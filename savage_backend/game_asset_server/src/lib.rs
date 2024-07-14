mod api;
mod config;
mod game_asset_dictionary;

use std::{path::PathBuf, sync::Arc};

use game_asset_dictionary::GameAssetDictionary;
use rocket::{http::Method, routes, Config};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

#[derive(Debug)]
pub enum Error {
    Unknown(String),
    AssetsRootDirectoryInvalid(String),
    AssetsMapFileInvalid(String),

}

#[derive(Debug)]
pub struct GameAssetServerConfig {
    pub server_base_path: String,
    pub port: Option<u16>,
    pub asset_root_directory: PathBuf,
    pub asset_map_file: PathBuf,
    pub allowed_origins_regex: Vec<&'static str>
}

pub struct GameAssetServer {
    server_base_path: String,
    port: Option<u16>,
    assets_repository: Arc<GameAssetDictionary>,
    allowed_origins: AllowedOrigins
}

impl GameAssetServer {
    pub fn new(config: GameAssetServerConfig) -> Result<GameAssetServer, Error> {
        if !std::path::Path::new(&config.asset_root_directory).is_dir() {
            return Err(Error::AssetsRootDirectoryInvalid(format!("Wrong assets root directory: {:?}", &config.asset_root_directory)));
        }
        let asset_directory = config.asset_root_directory;

        let asset_mapping_file = std::fs::File::open(config.asset_map_file);
        if let Err(err) = asset_mapping_file {
            return Err(Error::AssetsMapFileInvalid(err.to_string()));
        }
        let asset_mapping = serde_json::from_reader(asset_mapping_file.unwrap());
        if let Err(err) = asset_mapping {
            return Err(Error::AssetsMapFileInvalid(err.to_string()));
        }
        let asset_mapping = asset_mapping.unwrap();

        let server_base_path = config.server_base_path;
        let port = config.port;
        let assets_repository = Arc::new(GameAssetDictionary::new(asset_directory, asset_mapping));
        let allowed_origins = AllowedOrigins::some_regex(&config.allowed_origins_regex);

        Ok(GameAssetServer {
            server_base_path,
            port,
            assets_repository,
            allowed_origins
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
            .manage(self.assets_repository.clone())
            .mount(self.server_base_path.clone(),
                routes![
                    api::api::get_asset
                ]
            )
            .attach(cors.unwrap())
            .ignite().await.map_err(|e| Error::Unknown(format!("Ignition error: {}", e)))?
            .launch().await.map_err(|e| Error::Unknown(format!("Launch error: {}", e)))?;
        
        Ok(())
    }
}