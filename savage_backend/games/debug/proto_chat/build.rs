use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    let communications_defs_dir = PathBuf::from(std::env::var("COMMUNICATION_DEFINITIONS_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    build_protos_from_dir(
        &communications_defs_dir.join("debug/proto_chat"),
        &project_dir.join("src/.generated/proto"),
    )?;

    Ok(())
}