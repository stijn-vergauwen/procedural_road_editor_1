use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct List;

#[derive(Component, Clone, Copy)]
pub struct ListItem {
    // TODO: replace with iter_ancestors
    /// Reference to the List entity this item belongs to.
    pub list_entity: Entity,
    /// This item's order in the list.
    pub order: u8,
}

impl ListItem {
    pub fn new(list_entity: Entity, order: u8) -> Self {
        Self { list_entity, order }
    }

    /// Returns this ListItem's order as an index.  
    /// If this list is based on a data array this can be used to get the corresponding item.
    pub fn index(&self) -> usize {
        self.order as usize
    }
}
