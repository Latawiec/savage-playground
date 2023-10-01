use bevy::prelude::{Component, Entity};
use std::collections::HashMap;

pub type AggroLevel = u32;

#[derive(Component)]
pub struct AggroTable {
    aggro_table: HashMap<Entity, AggroLevel>,
    current_max: AggroLevel,
}

// TODO: Think - should I pass aggro changes as events? Sounds kinda cool.

impl AggroTable {
    pub fn increase_aggro(&mut self, target: &Entity, aggro_value: AggroLevel) {
        if !self.aggro_table.contains_key(&target) {
            self.aggro_table.insert(*target, Default::default());
        }

        let current_value = self.aggro_table.get_mut(&target).unwrap();
        *current_value += aggro_value;

        if *current_value > self.current_max {
            self.current_max = *current_value;
        }
    }

    pub fn set_aggro(&mut self, target: &Entity, aggro_value: AggroLevel) {
        if !self.aggro_table.contains_key(&target) {
            self.aggro_table.insert(*target, Default::default());
        }

        let current_value = self.aggro_table.get_mut(&target).unwrap();
        *current_value = aggro_value;

        if *current_value > self.current_max {
            self.current_max = *current_value;
        }
    }

    pub fn get_top_aggro_value(&self) -> AggroLevel {
        return self.current_max;
    }

    pub fn erase_aggro(&mut self, target: &Entity) {
        if !self.aggro_table.contains_key(&target) {
            self.aggro_table.remove(&target);
        }
    }
}
