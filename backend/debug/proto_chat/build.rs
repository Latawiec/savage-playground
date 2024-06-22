use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    build_protos_from_dir(
        &project_dir.join("proto"),
        &project_dir.join("src/.generated/proto"),
    )?;

    Ok(())
}