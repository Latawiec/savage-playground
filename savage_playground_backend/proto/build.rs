use std::{io::Result, path::{self, Path}, fs};


fn main() -> Result<()> {

    let protos_dir: String = std::env::var("BUILD_PROTO_DIR").unwrap();
    let protos_list = [
        fs::canonicalize(format!("{protos_dir}/game_message/renderer/gl_types.proto"))?,
        fs::canonicalize(format!("{protos_dir}/game_message/renderer/drawable.proto"))?,
        fs::canonicalize(format!("{protos_dir}/game_message/renderer/assets.proto"))?,
        fs::canonicalize(format!("{protos_dir}/game_message/renderer/vertex_attributes.proto"))?,
        fs::canonicalize(format!("{protos_dir}/game_message/renderer/uniform_attributes.proto"))?,
    ];

    let includes_list = [
        fs::canonicalize(format!("{protos_dir}/game_message/renderer/"))?,
        fs::canonicalize(format!("{protos_dir}/game_message/"))?,
    ];

    for proto_path in &protos_list {
        println!("cargo:rerun-if-changed={}", proto_path.to_str().unwrap());  
    }

    let proto_output: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let renderer_output_rel = format!("{proto_output}/src/game_message/renderer");
    fs::create_dir_all(&renderer_output_rel)?;
    let renderer_output_dir = fs::canonicalize(renderer_output_rel)?;

    let mut game_message_config = prost_build::Config::new();
    game_message_config.out_dir(&renderer_output_dir);
    game_message_config.compile_protos(&protos_list, &includes_list)?;

    Ok(())
}