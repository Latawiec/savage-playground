use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    // Generate proto
    let communication_dir = std::path::absolute(std::env::var("PROTO_COMMUNICATION_DIR").unwrap()).unwrap();
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    
    let protos_dir = std::path::absolute(communication_dir.join("game_interface")).unwrap();
    let output_dir = std::path::absolute(project_dir.join("src/.gen/proto")).unwrap();

    build_protos_from_dir(&protos_dir, &output_dir)?;

    Ok(())
}
