use std::{io::Result, path::{Path, PathBuf}, fs};

use typify::{TypeSpace, TypeSpaceSettings};

pub fn generate_mod_file(schema_files: &[PathBuf], output_dir: &Path) -> Result<()> {
    fs::create_dir_all(&output_dir)?;
    let mut contents = String::new();
    for schema_path in schema_files {
        let proto_file_name = schema_path.file_name().unwrap().to_str().unwrap().to_owned();
        let module_name = proto_file_name.strip_suffix(".json").unwrap();
        contents.push_str(&format!("pub mod {module_name};\n"));
    }

    fs::write(output_dir.join("mod.rs"), &contents)?;
    Ok(())
}

pub fn build_schemas_from_dir(source_rel_dir: &Path, output_rel_dir: &Path) -> Result<()> {
    // Check source exists.
    assert!(source_rel_dir.is_dir());

    // Iterate source dir looking for protos.
    let mut schemas_list = Vec::<PathBuf>::new();
    for entry in fs::read_dir(&source_rel_dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.metadata().unwrap().is_file() {
            continue;
        }

        let file_name = entry.file_name().into_string().unwrap();
        if file_name.ends_with(".json") {
            let proto_path = std::path::absolute(source_rel_dir.join(PathBuf::from(file_name)))?;
            // If any of these files changes, rebuild.
            println!("cargo:rerun-if-changed={}", proto_path.to_str().unwrap());  
            schemas_list.push(proto_path);
        }
    }

    // Prepare output.
    fs::create_dir_all(&output_rel_dir)?;
    let output_dir = std::path::absolute(&output_rel_dir)?;

    // Generate Rust from Schemnas
    for schema_path in &schemas_list {
        let content = std::fs::read_to_string(schema_path)?;
        let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)?;

        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
        type_space.add_root_schema(schema).unwrap();

        let generated = format!(
            "{}\n{}",
            "use serde::{Deserialize, Serialize};",
            prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
        );

        let out_file_name = schema_path.file_name().unwrap().to_str().unwrap().to_owned().strip_suffix(".json").unwrap().to_owned();
        let out_file = output_dir.join(format!("{}.rs", &out_file_name));
        fs::write(&out_file, generated)?;
    }

    generate_mod_file(&schemas_list, &output_dir)?;

    Ok(())
}