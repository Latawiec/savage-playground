use std::io::Result;
use std::path::PathBuf;

use schema_gen::build_schemas_from_dir;

fn main() -> Result<()> {

    // 
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let local_schema_dir = project_dir.join("schema");
    let output_dir = project_dir.join("src/config/.gen/schema");

    build_schemas_from_dir(&local_schema_dir, &output_dir)?;

    Ok(())
}
