use std::fs;
use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    let protos_dir: PathBuf = PathBuf::from(std::env::var("PROTO_GAME_INTERFACE_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    build_protos_from_dir(
        &protos_dir.join("game_output"),
        &project_dir.join("src/.generated/game_output"),
    )?;

    fs::write(project_dir.join("src/.generated/game_output/mod.rs"), format!("\
     pub mod message;\n\
     pub mod renderer;\n\
     pub mod settings;\n\
     pub mod ui;\n
    "
    ))?;

    fs::write(project_dir.join("src/.generated/mod.rs"), format!(
        "pub mod game_output;\n",
    ))?;
    
    Ok(())
}

