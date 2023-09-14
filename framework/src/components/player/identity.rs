use std::default::Default;
use uuid;
use bevy::ecs::component::Component;

#[derive(Component, PartialEq, Eq)]
pub struct Identity {
    name: String,
    uuid: uuid::Uuid,
}

impl Identity {
    pub fn new(name: String) -> Self {
        Identity {
            name,
            uuid: uuid::Uuid::now_v1(&[1, 2, 3, 4, 5, 6]),
        }
    }
}