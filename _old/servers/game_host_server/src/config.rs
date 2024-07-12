use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value_t = 0)]
    pub game_host_server_port: u16,
    #[arg(short, long, default_value_t = 0)]
    pub asset_server_port: u16,
    #[arg(short, long)]
    pub game_dir_mapping_file: String,
    #[arg(short, long)]
    pub server_name: String,
}
