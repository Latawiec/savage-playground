use std::path::PathBuf;

use crate::config::schema::asset_mapping::AssetMapping;

pub struct GameAssetDictionary {
    asset_directory: PathBuf,
    asset_mapping: AssetMapping,
}

impl GameAssetDictionary {
    pub fn new(asset_directory: PathBuf, asset_mapping: AssetMapping) -> GameAssetDictionary {
        GameAssetDictionary {
            asset_directory,
            asset_mapping
        }
    }

    pub async fn get_asset(&self, asset_name: &str) -> std::io::Result<rocket::fs::NamedFile> {
        let asset_relative_path = self.asset_mapping.get(asset_name);
        if let None = asset_relative_path {
            return Err(std::io::ErrorKind::NotFound.into());
        }
        let asset_absolute_path = self.asset_directory.join(asset_relative_path.unwrap());
        rocket::fs::NamedFile::open(asset_absolute_path).await
    }
}