use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub enum AggroChangeEvent {
    Taunt {
        player: Entity,
        target: Entity,
    },
    Reset {
        player: Entity,
        target: Entity,
    },
}
