use std::{path::{PathBuf, Path}, env, fs::{self, OpenOptions}, io};

pub fn get_output_path() -> Option<PathBuf> {
    //<root or manifest path>/target/<profile>/
    let output_dir_string = env::var("OUT_DIR").unwrap();
    let output_dir_path = Path::new(&output_dir_string);
    let build_dir_name = "build";

    let mut ancestors = output_dir_path.ancestors();
    while let Some(path) = ancestors.next() {
        if let Some(element) = path.file_name() {
            if element == build_dir_name {
                return Some(ancestors.next().unwrap().to_owned());
            }
        }
    }
    None
}

pub fn copy_file(src: &Path, dest: &Path) {
    // Create all intermediate folders if they don't exist.
    {
        let _ = fs::create_dir_all(dest.parent().unwrap());
    }

    // Create file if it doesn't exist 
    {
        let file = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(&dest);
        match &file {
            Ok(_) => {},
            Err(err) => {
                println!("Failed to create file: {}", err);
            },
        }
        drop(file);
    }

    println!("Copying {} to {}", src.to_str().unwrap(), dest.to_str().unwrap());
    fs::copy(src, dest).unwrap();
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}