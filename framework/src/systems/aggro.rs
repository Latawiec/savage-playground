use bevy::prelude::{EventReader, Query};

use crate::{
    components::boss::aggro_table::{AggroLevel, AggroTable},
    events::aggro::AggroChangeEvent,
};

pub fn aggro_system(
    mut ev_aggro: EventReader<AggroChangeEvent>,
    mut query_aggro_table: Query<&mut AggroTable>,
) {
    const TAUNT_OVERTAKE_MULTIPLIER: f32 = 1.1;

    for ev in ev_aggro.iter() {
        match ev {
            AggroChangeEvent::Change {
                player,
                target,
                value,
            } => {
                if let Ok(mut target_aggro_table) = query_aggro_table.get_mut(*target) {
                    target_aggro_table.increase_aggro(player, *value);
                } else {
                    tracing::warn!("Aggro change requested on entity without an aggro table.");
                }
            }
            AggroChangeEvent::Erase { player, target } => {
                if let Ok(mut target_aggro_table) = query_aggro_table.get_mut(*target) {
                    target_aggro_table.erase_aggro(player);
                } else {
                    tracing::warn!("Aggro erase requested on entity without an aggro table.");
                }
            }
            AggroChangeEvent::Taunt { player, target } => {
                if let Ok(mut target_aggro_table) = query_aggro_table.get_mut(*target) {
                    let current_top = target_aggro_table.get_top_aggro_value();
                    target_aggro_table.set_aggro(
                        player,
                        (current_top as f32 * TAUNT_OVERTAKE_MULTIPLIER) as AggroLevel,
                    );
                } else {
                    tracing::warn!("Aggro taunt requested on entity without an aggro table.");
                }
            }
        }
    }
}
