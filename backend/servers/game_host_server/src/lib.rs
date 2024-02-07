use clap::Parser;
use config::Config;
// use game_host::game_host_manager::GameHostManagerHandle;
// use room_server::server::RoomServerHandle;
use tokio::task::JoinHandle;
use std::path::PathBuf;

pub mod config;
// pub mod game_host;
mod instance;
// mod game_host;
// mod room_server;
pub mod room_host;
pub mod game_launcher;

// Starts both Asset Server and Game Host Server
pub struct BundledServer {
    room_server_join_handle: JoinHandle<()>,
    asset_server_join_handle: JoinHandle<()>,
    // game_host: GameHostManagerHandle,
}

impl BundledServer {
    // pub fn new() -> Option<BundledServer> {
    //     let args = Config::parse();
    //     Self::new_with_config(args)
    // }

    // pub fn new_with_config(args: Config) -> Option<BundledServer> {
    //     let (room_server, room_server_join_handle) = RoomServerHandle::new(([127, 0, 0, 1], args.game_host_server_port).into());
    //     let (asset_Server, asset_server_join_handle) = AssetServerHandle::new(([127, 0, 0, 1], args.asset_server_port).into());

    //     let game_dir_mapping_file = PathBuf::from(args.game_dir_mapping_file);
    //     let game_host = GameHostManagerHandle::new(room_server.clone(), &game_dir_mapping_file).unwrap();

    //     Some(BundledServer { room_server_join_handle, asset_server_join_handle, game_host })
    // }

    pub async fn serve(self) {
        let _ = tokio::join!(self.room_server_join_handle, self.asset_server_join_handle);
    }
}