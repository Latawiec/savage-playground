use std::collections::BTreeSet;

use bevy::prelude::Entity;

#[derive(Default)]
pub struct EntitySetTracket {
    pub entities: BTreeSet<Entity>,
    pub just_added: BTreeSet<Entity>,
    pub just_removed: BTreeSet<Entity>,
}

impl EntitySetTracket {
    pub fn update(&mut self, entities: BTreeSet<Entity>) {
        self.just_added = &entities - &self.entities;
        self.just_removed = &self.entities - &entities;
        self.entities = entities;
    }
}
