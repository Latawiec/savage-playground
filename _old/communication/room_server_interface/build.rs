use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;
use schema_gen::build_schemas_from_dir;

fn main() -> Result<()> {
    // Generate proto
    let protos_dir = PathBuf::from(std::env::var("PROTO_ROOM_SERVER_INTERFACE_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    build_protos_from_dir(&protos_dir, &project_dir.join("src/.generated/proto"))?;

    // Generate schema
    let schemas_dir = PathBuf::from(std::env::var("SCHEMA_ROOM_SERVER_INTERFACE_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    build_schemas_from_dir(&schemas_dir, &project_dir.join("src/.generated/schema"))?;

    Ok(())
}
