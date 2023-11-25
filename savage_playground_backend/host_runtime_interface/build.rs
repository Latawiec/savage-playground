use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    let protos_dir = PathBuf::from(std::env::var("PROTO_HOST_RUNTIME_INTERFACE_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    build_protos_from_dir(&protos_dir, &project_dir.join("src/.generated"))?;

    Ok(())
}
