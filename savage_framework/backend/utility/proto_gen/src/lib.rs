use std::{io::Result, path::{Path, PathBuf}, fs};

// This doesn't actually care about what files were generated.
// It will iterate over all files in the output dir, look for .rs files and make a module of them.
pub fn generate_mod_file(output_dir: &Path) -> Result<()> {
    let mut contents = String::new();
    for entry in fs::read_dir(&output_dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.metadata().unwrap().is_file() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if file_name.ends_with(".rs") && file_name.ne("mod.rs") {
            contents.push_str(&format!("pub mod {};\n", file_name.strip_suffix(".rs").unwrap()));
        }
    }

    fs::write(output_dir.join("mod.rs"), &contents)?;
    Ok(())
}

pub fn build_protos_from_dir(source_rel_dir: &Path, output_rel_dir: &Path, include_rel_dirs: &[&Path]) -> Result<()> {
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
            let proto_path = std::path::absolute(source_rel_dir.join(PathBuf::from(file_name)))?;
            // If any of these files changes, rebuild.
            println!("cargo:rerun-if-changed={:?}", &proto_path);  
            protos_list.push(proto_path);
        }
    }

    // I think this is proto-path
    let mut includes_list = vec![
        std::path::absolute(source_rel_dir)?
    ];

    for include_dir in include_rel_dirs {
        includes_list.push(std::path::absolute(include_dir.to_path_buf())?)
    }

    println!("Includes list: {:?}", &includes_list);

    // Prepare output.
    fs::create_dir_all(&output_rel_dir)?;
    let output_dir = std::path::absolute(&output_rel_dir)?;

    let mut protos_config = prost_build::Config::new();
    protos_config.out_dir(&output_dir);
    protos_config.compile_protos(&protos_list, &includes_list)?;

    generate_mod_file(&output_dir)?;

    Ok(())
}