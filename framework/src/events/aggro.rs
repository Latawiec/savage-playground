use bevy::prelude::{Entity, Event};

use crate::components::boss::aggro_table::AggroLevel;

#[derive(Event)]
pub enum AggroChangeEvent {
    Change {
        player: Entity,
        target: Entity,
        value: AggroLevel,
    },
    Taunt {
        player: Entity,
        target: Entity,
    },
    Erase {
        player: Entity,
        target: Entity,
    },
}
