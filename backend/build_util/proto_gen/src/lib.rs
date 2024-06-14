use std::{io::Result, path::{Path, PathBuf}, fs};

pub fn generate_mod_file(proto_files: &[PathBuf], output_dir: &Path) -> Result<()> {
    fs::create_dir_all(&output_dir)?;
    let mut contents = String::new();
    for proto_path in proto_files {
        println!("cargo:warning={:?}", proto_path);
        let proto_file_name = proto_path.file_name().unwrap().to_str().unwrap().to_owned();
        let module_name = proto_file_name.strip_suffix(".proto").unwrap();
        contents.push_str(&format!("pub mod {module_name};\n"));
    }

    fs::write(output_dir.join("mod.rs"), &contents)?;
    Ok(())
}

pub fn build_protos_from_dir(source_rel_dir: &Path, output_rel_dir: &Path) -> Result<()> {
    // Check source exists.
    assert!(source_rel_dir.is_dir());

    // Iterate source dir looking for protos.
    let mut protos_list = Vec::<PathBuf>::new();
    for entry in fs::read_dir(&source_rel_dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.metadata().unwrap().is_file() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if file_name.ends_with(".proto") {
            let proto_path = fs::canonicalize(source_rel_dir.join(PathBuf::from(file_name)))?;
            // If any of these files changes, rebuild.
            println!("cargo:rerun-if-changed={}", proto_path.to_str().unwrap());  
            protos_list.push(proto_path);
        }
    }

    // I think this is proto-path
    let includes_list = [
        fs::canonicalize(source_rel_dir)?
    ];

    // Prepare output.
    fs::create_dir_all(&output_rel_dir)?;
    let output_dir = fs::canonicalize(&output_rel_dir)?;

    let mut protos_config = prost_build::Config::new();
    protos_config.out_dir(&output_dir);
    protos_config.compile_protos(&protos_list, &includes_list)?;

    generate_mod_file(&protos_list, &output_dir)?;

    Ok(())
}