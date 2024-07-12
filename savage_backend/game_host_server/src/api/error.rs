use rocket::response::Responder;


#[derive(Responder)]
pub enum APIError {
    #[response(status = 500)]
    Bad(String),

    #[response(status = 404)]
    RoomNotFound(String),
}