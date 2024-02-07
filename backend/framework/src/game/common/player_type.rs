use uuid;

pub type PlayerID = uuid::Uuid;
pub fn new_player_id() -> PlayerID {
    uuid::Uuid::now_v1(&[1, 2, 3, 4, 5, 6])
}
