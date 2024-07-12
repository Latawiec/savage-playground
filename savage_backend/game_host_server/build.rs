use std::io::Result;
use std::path::PathBuf;

use schema_gen::build_schemas_from_dir;

fn main() -> Result<()> {

    // Generate schema
    let communication_dir = std::path::absolute(std::env::var("SCHEMA_COMMUNICATION_DIR").unwrap()).unwrap();
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let schemas_dir = std::path::absolute(communication_dir.join("host_interface")).unwrap();
    let output_dir = std::path::absolute(project_dir.join("src/game_host/interface/.gen/schema")).unwrap();

    build_schemas_from_dir(&schemas_dir, &output_dir)?;

    // 
    let local_schema_dir = project_dir.join("schema");
    let output_dir = project_dir.join("src/config/.gen/schema");

    build_schemas_from_dir(&local_schema_dir, &output_dir)?;

    Ok(())
}
