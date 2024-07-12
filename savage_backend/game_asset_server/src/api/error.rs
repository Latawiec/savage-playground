use rocket::response::Responder;


#[derive(Responder)]
pub enum GameAssetServerError {
    #[response(status = 500)]
    Bad(String),

    #[response(status = 404)]
    AssetNotFound(String),
}