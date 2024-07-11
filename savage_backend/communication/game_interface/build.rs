use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    // Generate proto
    let communication_dir = PathBuf::from(std::env::var("SAVAGE_COMMUNICATION_DIR").unwrap());
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    
    let protos_dir = communication_dir.join("game_room_interface/proto");
    let output_dir = project_dir.join("src/.gen/proto");

    build_protos_from_dir(&protos_dir, &output_dir)?;

    Ok(())
}
