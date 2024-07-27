use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

const FRAMEWORK_PROTO_DIR: &'static str = "../../proto";

fn main() -> Result<()> {
    // Generate proto
    let framework_proto_dir = std::path::absolute(FRAMEWORK_PROTO_DIR).unwrap();
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    
    let protos_dir = std::path::absolute(framework_proto_dir.join("game_renderer")).unwrap();
    let output_dir = std::path::absolute(project_dir.join("src/.gen/proto")).unwrap();

    build_protos_from_dir(&protos_dir, &output_dir, &[])?;

    Ok(())
}
