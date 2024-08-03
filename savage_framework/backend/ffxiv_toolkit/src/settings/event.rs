use bevy::prelude::Event;

#[derive(Event)]
pub struct PlayerJoinedEvent {
    pub player_id: u64,
}

#[derive(Event)]
pub struct PlayerLeftEvent {
    pub player_id: u64,
}
