use bevy::prelude::Component;

#[derive(Component)]
pub struct MechanicInfo {
    pub mechanic_id: u64,
    pub name: String,
    pub short_name: Option<String>,
}
