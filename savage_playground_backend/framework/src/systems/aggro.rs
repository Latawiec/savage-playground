use bevy::prelude::{EventReader, Query};

use crate::{
    components::boss::aggro_table::AggroTable,
    events::aggro::AggroChangeEvent,
};

pub fn aggro_system(
    mut ev_aggro: EventReader<AggroChangeEvent>,
    mut query_aggro_table: Query<&mut AggroTable>,
) {
    const TAUNT_OVERTAKE_MULTIPLIER: f32 = 1.1;
    const TAUNT_FIRST_TARGET_DEFAULT_VALUE: u32 = 1;

    for ev in ev_aggro.iter() {
        match ev {
            AggroChangeEvent::Reset { player, target } => {
                if let Ok(mut target_aggro_table) = query_aggro_table.get_mut(*target) {
                    target_aggro_table.erase_aggro(player);
                } else {
                    tracing::warn!("Aggro erase requested on entity without an aggro table.");
                }
            }
            AggroChangeEvent::Taunt { player, target } => {
                if let Ok(mut target_aggro_table) = query_aggro_table.get_mut(*target) {
                    let current_top = target_aggro_table.get_nth_top_aggro_entry(0);
                    match current_top {
                        Some(top) => {
                            let current_aggro = target_aggro_table.get_aggro_for_entity(*target).unwrap_or(0);
                            let increase = (top.aggro as f32 * TAUNT_OVERTAKE_MULTIPLIER) as u32 - current_aggro;
                            target_aggro_table.increase_aggro(player, increase);
                        },
                        None => target_aggro_table.increase_aggro(player, TAUNT_FIRST_TARGET_DEFAULT_VALUE)
                    };

                    
                } else {
                    tracing::warn!("Aggro taunt requested on entity without an aggro table.");
                }
            }
        }
    }
}
