use clap::Parser;

pub struct Error(pub String);

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct ServerConfig {
    #[arg(short, long, default_value_t = 0)]
    port: u32,
    #[arg(short, long)]
    server_name: String,
}

impl ServerConfig {
    pub fn parse_config() -> Result<ServerConfig, Error> {
        match ServerConfig::try_parse() {
            Ok(config) => Ok(config),
            Err(error) => Err(Error(format!("{}", error))),
        }
    }
}
