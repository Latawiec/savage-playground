use std::{env, error};

use clap::Parser;

pub struct ProgramConfigError(pub String);

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct ProgramConfig {
    #[arg(short, long)]
    port: u32,
    #[arg(short, long)]
    world: String,
}

impl ProgramConfig {
    pub fn parse_config() -> Result<ProgramConfig, ProgramConfigError> {
        match ProgramConfig::try_parse() {
            Ok(config) => Ok(config),
            Err(error) => {
                Err(ProgramConfigError(format!("{}", error)))
            },
        }
    }
}
