use bevy::prelude::{Component, Entity};
use std::{cmp::Ordering, collections::HashMap};

pub type AggroLevel = u32;

#[derive(Clone, Copy)]
pub struct AggroTableEntry {
    pub aggro: AggroLevel,
    pub entity: Entity,
}

#[derive(Component, Default)]
pub struct AggroTable {
    aggro_table: Vec<AggroTableEntry>,
    entity_to_vec_idx: HashMap<Entity, usize>,
}

impl AggroTable {
    pub fn increase_aggro(&mut self, target: &Entity, aggro_value: AggroLevel) {
        let entity_idx = self.entity_to_vec_idx.get(&target);
        let mut entry = match entity_idx {
            Some(idx) => *self.aggro_table.get(*idx).unwrap(),
            None => AggroTableEntry {
                aggro: Default::default(),
                entity: *target,
            },
        };
        let target = entry.aggro + aggro_value;

        let (Ok(dest_idx) | Err(dest_idx)): Result<usize, _> =
            self.aggro_table
                .binary_search_by(|e| match e.aggro.cmp(&target) {
                    // Since we try to find position of first element that is lower,
                    // we treat all equal values as lower to move left.
                    Ordering::Greater => Ordering::Less,
                    Ordering::Equal => Ordering::Less,
                    Ordering::Less => Ordering::Greater,
                });

        match entity_idx {
            Some(idx) => {
                self.aggro_table.get_mut(*idx).unwrap().aggro += aggro_value;
                if *idx == dest_idx {
                    return;
                }
                for i in ((dest_idx + 1)..=*idx).rev() {
                    self.aggro_table.swap(i, i - 1);
                }

                for i in dest_idx..*idx {
                    *self
                        .entity_to_vec_idx
                        .get_mut(&self.aggro_table.get(i).unwrap().entity)
                        .unwrap() = i;
                }
            }
            None => {
                entry.aggro += aggro_value;
                self.aggro_table.insert(dest_idx, entry);
                for i in dest_idx..self.aggro_table.len() {
                    let _ = self
                        .entity_to_vec_idx
                        .insert(self.aggro_table.get(i).unwrap().entity, i);
                }
            }
        }
    }

    pub fn get_nth_top_aggro_entry(&self, idx: usize) -> Option<&AggroTableEntry> {
        self.aggro_table.get(idx)
    }

    pub fn get_aggro_for_entity(&self, entity: Entity) -> Option<AggroLevel> {
        let idx = self.entity_to_vec_idx.get(&entity)?;
        Some(self.aggro_table.get(*idx)?.aggro)
    }

    pub fn erase_aggro(&mut self, target: &Entity) {
        if let Some(idx) = self.entity_to_vec_idx.remove(&target) {
            self.aggro_table.remove(idx);
            for i in idx..self.aggro_table.len() {
                let _ = self
                    .entity_to_vec_idx
                    .insert(self.aggro_table.get(i).unwrap().entity, i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::entity::Entity;

    use crate::game::aggro::component::AggroTable;

    #[test]
    fn aggro_test() {
        let mut tab = AggroTable::default();

        let e0 = Entity::from_raw(0);
        let e1 = Entity::from_raw(1);
        let e2 = Entity::from_raw(2);
        let e3 = Entity::from_raw(3);
        let e4 = Entity::from_raw(4);
        let e5 = Entity::from_raw(5);

        tab.increase_aggro(&e0, 1);
        tab.increase_aggro(&e1, 2);
        tab.increase_aggro(&e2, 4);
        tab.increase_aggro(&e3, 5);
        tab.increase_aggro(&e4, 3);
        tab.increase_aggro(&e0, 4);
        tab.increase_aggro(&e5, 0);

        assert_eq!(tab.get_nth_top_aggro_entry(0).unwrap().aggro, 5);
        assert_eq!(tab.get_nth_top_aggro_entry(1).unwrap().aggro, 5);
        assert_eq!(tab.get_nth_top_aggro_entry(2).unwrap().aggro, 4);
        assert_eq!(tab.get_nth_top_aggro_entry(3).unwrap().aggro, 3);
        assert_eq!(tab.get_nth_top_aggro_entry(4).unwrap().aggro, 2);
        assert_eq!(tab.get_nth_top_aggro_entry(5).unwrap().aggro, 0);
        assert_eq!(tab.get_nth_top_aggro_entry(3).unwrap().entity, e4);
    }
}
