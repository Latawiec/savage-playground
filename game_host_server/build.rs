use std::{path::{PathBuf, Path}, env, fs::{self, OpenOptions}};

fn get_output_path() -> Option<PathBuf> {
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


fn main() {

    //let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = get_output_path().unwrap();
    let src = Path::join(&env::current_dir().unwrap(), "src/game_host/game_dir_mapping.json");
    let dest = Path::join(Path::new(&target_dir), "assets/game_dir_mapping.json");

    // Create all intermediate folders if they don't exist.
    {
        let _ = fs::create_dir_all(dest.parent().unwrap());
    }

    // Create file if it doesn't exist 
    {
        let file = OpenOptions::new()
                                .write(true)
                                .create_new(true)
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