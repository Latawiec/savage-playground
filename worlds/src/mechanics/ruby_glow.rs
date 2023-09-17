use bevy::{prelude::{Component, Commands}, transform::TransformBundle};


#[derive(Component)]
pub enum Crystal {
    Topaz,
    Venom
}

pub struct TopazCrystal {}

impl TopazCrystal {
    pub fn spawn(commands: &mut Commands) {
        commands.spawn(Crystal::Topaz)
            .insert(TransformBundle::default())
            ;
    }
}

pub struct VenomCrystal {}