
#[derive(Event)]
pub struct MechanicFailedEvent {
    pub player_id: u64,
    pub mechanic_id: u64,
    pub reason: String,
}