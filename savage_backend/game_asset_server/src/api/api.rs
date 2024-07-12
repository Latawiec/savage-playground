use super::error::GameAssetServerError;
use crate::game_asset_dictionary::GameAssetDictionary;
use rocket::{fs::NamedFile, get, State};
use std::{path::PathBuf, sync::Arc};

#[get("/get_asset/<asset_path..>")]
pub async fn get_asset(
    asset_path: PathBuf,
    assets_repository: &State<Arc<GameAssetDictionary>>,
) -> Result<NamedFile, GameAssetServerError> {
    match assets_repository
        .get_asset(&asset_path.to_string_lossy())
        .await
    {
        Ok(file) => Ok(file),
        Err(err) => Err(GameAssetServerError::AssetNotFound(err.to_string())),
    }
}
