use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerIdentity {
    pub player_id: u64,
    pub player_name: Optional<String>,
}
