use bevy::ecs::component::Component;

#[derive(Component, PartialEq, Eq)]
pub enum Class {
    Tank,
    PureHealer,
    ShieldHealer,
    MeleeDamage,
    PhysicalRangedDamage,
    MagicalRangedDamage,
}