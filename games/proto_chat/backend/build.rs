use std::io::Result;
use std::path::PathBuf;

use proto_gen::build_protos_from_dir;

fn main() -> Result<()> {
    let project_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let local_proto_dir = project_dir.join("../proto");
    let output_dir = project_dir.join("src/.gen/proto");

    build_protos_from_dir(
        &local_proto_dir,
        &output_dir,
        &[]
    )?;

    Ok(())
}