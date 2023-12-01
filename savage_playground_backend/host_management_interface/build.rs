use std::io::Result;
use std::path::PathBuf;

use schema_gen::build_schemas_from_dir;

fn main() -> Result<()> {
    let schemas_dir = PathBuf::from(std::env::var("SCHEMA_HOST_MANAGEMENT_INTERFACE_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    build_schemas_from_dir(&schemas_dir, &project_dir.join("src/.generated"))?;

    Ok(())
}