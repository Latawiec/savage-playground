#[derive(Clone, Debug)]
pub enum Message {
    Text { data: String },
    Binary { data: Vec<u8> },
}
