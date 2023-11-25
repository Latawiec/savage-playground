use std::fs;
use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    let protos_dir: PathBuf = PathBuf::from(std::env::var("PROTO_GAME_INTERFACE_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    build_protos_from_dir(
        &protos_dir.join("game_output/renderer"),
        &project_dir.join("src/.generated/game_output/renderer"),
    )?;
    build_protos_from_dir(
        &protos_dir.join("game_output/settings"),
        &project_dir.join("src/.generated/game_output/settings"),
    )?;
    build_protos_from_dir(
        &protos_dir.join("game_output/ui"),
        &project_dir.join("src/.generated/game_output/ui"),
    )?;

    fs::write(project_dir.join("src/.generated/game_output/mod.rs"), format!("\
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

