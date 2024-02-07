use std::{env, path::Path};

use build_io::{copy_file, get_output_path};

fn copy_game_dir_mapping() {
    let target_dir = get_output_path().unwrap();
    let src = Path::join(
        &env::current_dir().unwrap(),
        "src/game_host/game_dir_mapping.json",
    );
    let dest = Path::join(Path::new(&target_dir), "assets/game_dir_mapping.json");

    copy_file(&src, &dest);
}

// fn copy_game_asset_mapping() {
//     let target_dir = get_output_path().unwrap();
//     let src = Path::join(&env::current_dir().unwrap(), "src/game_host/game_assets_mapping.json");
//     let dest = Path::join(Path::new(&target_dir), "assets/game_assets_mapping.json");

//     copy_file(&src, &dest);
// }

fn main() {
    // copy_game_dir_mapping();
    //copy_game_asset_mapping();
}
