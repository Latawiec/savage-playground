use bevy::ecs::component::Component;

#[derive(Component, PartialEq, Eq)]
pub enum RaidRole {
    MT,
    OT,
    H1,
    H2,
    M1,
    M2,
    R1,
    R2,
}
