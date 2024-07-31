use std::path::PathBuf;

use game_asset_server::{GameAssetServer, GameAssetServerConfig};
use game_host_server::{GameHostServer, GameHostServerConfig};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .init();


    let root: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let asset_root_directory = root.join("assets/");
    let games_root_directory: PathBuf = std::env::var("GAMES_ROOT_DIR").unwrap().into();
    let asset_map_file = root.join("assets/asset_mapping.json");
    let games_map_file = root.join("assets/games_mapping.json");

    let host_server_config = GameHostServerConfig {
        server_base_path: "/host".to_string(),
        port: Some(8001),
        games_root_directory,
        games_map_file,
        allowed_origins_regex: vec![".*"],
    };

    let asset_server_config = GameAssetServerConfig {
        server_base_path: "/asset".to_string(),
        port: Some(8002),
        asset_root_directory,
        asset_map_file,
        allowed_origins_regex: vec![".*"],
    };

    let host_server = GameHostServer::new(host_server_config);
    let asset_server = GameAssetServer::new(asset_server_config);

    if let Err(err) = host_server {
        tracing::error!("Failed to start up host server: {:?}", err);
        return;
    }

    if let Err(err) = asset_server {
        tracing::error!("Failed to start up asset server: {:?}", err);
        return;
    }

    let host_server = host_server.unwrap();
    let asset_server = asset_server.unwrap();

    let mut join_set = JoinSet::new();
    join_set.spawn(async move {
        if let Err(error) = host_server.serve().await {
            eprintln!("Host server error: {:?}", error);
        }
    });
    join_set.spawn(async move {
        if let Err(error) = asset_server.serve().await {
            eprintln!("Asset server error: {:?}", error);
        }
    });

    join_set.join_next().await;
}
