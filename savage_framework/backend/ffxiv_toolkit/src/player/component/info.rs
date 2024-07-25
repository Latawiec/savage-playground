use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerInfo {
    pub player_id: u64,
    pub player_name: Optional<String>,
}
