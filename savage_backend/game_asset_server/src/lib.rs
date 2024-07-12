mod api;
mod config;
mod game_asset_dictionary;

use std::{path::PathBuf, sync::Arc};

use game_asset_dictionary::GameAssetDictionary;
use rocket::routes;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
    AssetsRootDirectoryInvalid(String),
    AssetsMapFileInvalid(String),

}

#[derive(Debug)]
pub struct GameAssetServerConfig {
    pub server_base_path: String,
    pub assets_root_directory: PathBuf,
    pub assets_map_file: PathBuf,
}

pub struct GameAssetServer {
    server_base_path: String,
    assets_repository: Arc<GameAssetDictionary>,
}

impl GameAssetServer {
    pub fn new(config: GameAssetServerConfig) -> Result<GameAssetServer, Error> {
        if !std::path::Path::new(&config.assets_root_directory).is_dir() {
            return Err(Error::AssetsRootDirectoryInvalid(format!("Wrong assets root directory: {:?}", &config.assets_root_directory)));
        }
        let asset_directory = config.assets_root_directory;

        let asset_mapping_file = std::fs::File::open(config.assets_map_file);
        if let Err(err) = asset_mapping_file {
            return Err(Error::AssetsMapFileInvalid(err.to_string()));
        }
        let asset_mapping = serde_json::from_reader(asset_mapping_file.unwrap());
        if let Err(err) = asset_mapping {
            return Err(Error::AssetsMapFileInvalid(err.to_string()));
        }
        let asset_mapping = asset_mapping.unwrap();

        let server_base_path = config.server_base_path;
        let assets_repository = Arc::new(GameAssetDictionary::new(asset_directory, asset_mapping));
        Ok(GameAssetServer {
            server_base_path,
            assets_repository
        })
    }

    pub async fn serve(&self) -> Result<(), Error> {
        rocket::build()
            .manage(self.assets_repository.clone())
            .mount(self.server_base_path.clone(),
                routes![
                    api::api::get_asset
                ]
            )
            .ignite().await.map_err(|e| Error::Unknown(format!("Ignition error: {}", e)))?
            .launch().await.map_err(|e| Error::Unknown(format!("Launch error: {}", e)))?;
        
        Ok(())
    }
}