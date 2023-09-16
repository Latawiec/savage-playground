use bevy::ecs::component::Component;

use crate::types::player::PlayerID;

#[derive(Component, PartialEq, Eq)]
pub struct Identity {
    pub name: String,
    pub id: PlayerID,
}

impl Identity {
    pub fn new(name: String, id: PlayerID) -> Self {
        Identity { name, id }
    }
}
