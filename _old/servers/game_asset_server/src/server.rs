use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tokio::task::JoinHandle;
use warp::{
    filters::{header::Conditionals, path::Tail},
    Filter,
};

mod error {
    use warp::reject::Reject;

    #[derive(Debug)]
    pub enum Error {
        AssetNotFound,
        // InternalError,
    }

    impl Reject for Error {}
}

#[derive(Clone)]
pub struct AssetServerHandle {
    pub addr: SocketAddr,
    assets_dir: Arc<PathBuf>,
    asset_mapping: Arc<RwLock<HashMap<String, PathBuf>>>,
}

impl AssetServerHandle {
    pub fn new(server_address: SocketAddr) -> (AssetServerHandle, JoinHandle<()>) {
        const ASSETS_SERVER_ASSETS_DIR_ENV_VAR: &str = "ASSETS_SERVER_ASSETS_DIR";
        const ASSETS_MAPPING_FILE_ENV_VAR: &str = "ASSETS_SERVER_MAPPING_FILE";

        let assets_dir = match std::env::var(ASSETS_SERVER_ASSETS_DIR_ENV_VAR) {
            Ok(path) => path,
            Err(_) => {
                tracing::error!(
                    "{} undefined. Probably won't find any assets because of that.",
                    ASSETS_SERVER_ASSETS_DIR_ENV_VAR
                );
                "".to_owned()
            }
        };
        let assets_dir = PathBuf::from(assets_dir);
        if !assets_dir.exists() {
            tracing::error!("Provided assets folder does not exist: {:?}", assets_dir);
        }

        let asset_mapping_file = std::env::var(ASSETS_MAPPING_FILE_ENV_VAR);

        if asset_mapping_file.is_err() {
            tracing::error!("ASSETS_MAPPING_FILE_ENV_VAR is empty. Mapping not found. This server won't serve a thing.");
        }

        let asset_mapping_file = std::fs::File::open(asset_mapping_file.unwrap());
        if asset_mapping_file.is_err() {
            tracing::error!(
                "Couldn't open assets mapping file: {:?}. Mapping remains empty.",
                asset_mapping_file
            );
        }

        let asset_mapping = serde_json::from_reader::<std::fs::File, HashMap<String, PathBuf>>(
            asset_mapping_file.unwrap(),
        );
        if asset_mapping.is_err() {
            tracing::error!("Couldn't deserialize assets mapping. Mapping remains empty.");
        }

        let asset_mapping = asset_mapping.unwrap();
        println!("{:?}", asset_mapping);

        let server_handle = AssetServerHandle {
            addr: server_address,
            assets_dir: Arc::new(assets_dir),
            asset_mapping: Arc::new(RwLock::new(asset_mapping)),
        };

        let server_handle_clone = server_handle.clone();
        let server_handle_filter = warp::any().map(move || server_handle_clone.clone());

        let get_asset = warp::path("get_asset")
            .and(warp::path::tail())
            .and(warp::addr::remote())
            .and(warp::header::conditionals())
            .and(server_handle_filter)
            .and_then(Self::get_asset)
            .map(|reply| warp::reply::with_header(reply, "Access-Control-Allow-Origin", "http://localhost:8080")); // TODO: Learn more about CORS.

        let routes = get_asset;

        let server_join_handle = tokio::spawn(async move {
            warp::serve(routes).bind(server_address).await;
        });

        (server_handle, server_join_handle)
    }

    async fn get_asset(
        asset_name: Tail,
        _addr: Option<SocketAddr>,
        conditionals: Conditionals,
        server_handle: AssetServerHandle,
    ) -> Result<warp::fs::File, warp::Rejection> {
        tracing::info!("Requested: {:?}", asset_name);
        //let asset_path = server_handle.asset_mapping.read().unwrap().get(&asset_name).cloned();
        let asset_path = match server_handle
            .asset_mapping
            .read()
            .unwrap()
            .get(asset_name.as_str())
            .cloned()
        {
            Some(path) => Some(path),
            None => {
                tracing::warn!("Requested non-existing asset: {}", asset_name.as_str());
                None
            }
        };
        if asset_path.is_none() {
            return Err(warp::reject::custom(error::Error::AssetNotFound));
        }

        let asset_path = server_handle.assets_dir.clone().join(asset_path.unwrap());
        if !asset_path.exists() {
            tracing::warn!("File does not exist: {}", asset_name.as_str());
            return Err(warp::reject::custom(error::Error::AssetNotFound));
        }

        warp::reply::file(asset_path, conditionals).await
    }
}
