use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    // Generate proto
    println!("{:?}", std::env::var("PROTO_FRAMEWORK_DIR"));
    let framework_proto_dir = std::path::absolute(std::env::var("PROTO_FRAMEWORK_DIR").unwrap()).unwrap();
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    
    let protos_dir = std::path::absolute(framework_proto_dir.join("ffxiv_toolkit")).unwrap();
    let output_dir = std::path::absolute(project_dir.join("src/.gen/proto")).unwrap();

    build_protos_from_dir(&protos_dir, &output_dir, &[
        &framework_proto_dir
    ])?;

    Ok(())
}
