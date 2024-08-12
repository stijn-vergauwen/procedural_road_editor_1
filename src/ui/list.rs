use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct List;

#[derive(Component, Clone, Copy)]
pub struct ListItem {
    // TODO: replace with iter_ancestors
    /// Reference to the List entity this item belongs to.
    list_entity: Entity,
    /// This item's order in the list.
    index: usize,
}

impl ListItem {
    pub fn new(list_entity: Entity, index: usize) -> Self {
        Self { list_entity, index }
    }

    pub fn list_entity(&self) -> Entity {
        self.list_entity
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}
