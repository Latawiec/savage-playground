use std::path::PathBuf;

use game_asset_server::{GameAssetServer, GameAssetServerConfig};



#[tokio::main]
async fn main() {
    let root: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let assets_root_directory = root.join("assets/");
    let assets_map_file = root.join("assets/asset_mapping.json");

    let config = GameAssetServerConfig {
        server_base_path: "/test".to_string(),
        assets_root_directory,
        assets_map_file,
    };
    match GameAssetServer::new(config) {
        Ok(server) => {
            if let Err(err) = server.serve().await {
                eprintln!("Error: {:?}", err);
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        },
    }    
}
