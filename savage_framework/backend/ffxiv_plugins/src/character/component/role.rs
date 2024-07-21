use bevy::ecs::component::Component;

#[derive(Component, PartialEq, Eq)]
pub enum Role {
    Tank,
    Healer,
    DPS,
}