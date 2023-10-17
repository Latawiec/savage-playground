use bevy::prelude::{Entity, Event};

use crate::components::boss::aggro_table::AggroLevel;

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
