use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct List;

#[derive(Component, Clone, Copy)]
pub struct ListItem {
    /// Reference to the List entity this item belongs to.
    pub list_entity: Entity,
    /// This item's order in the list.
    pub order: u8,
}

impl ListItem {
    pub fn new(list_entity: Entity, order: u8) -> Self {
        Self { list_entity, order }
    }
}
