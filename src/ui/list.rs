pub mod add_list_item;
pub mod delete_list_item;
pub mod reorder_button;
pub mod reorder_list;

use add_list_item::AddListItemPlugin;
use bevy::prelude::*;
use delete_list_item::DeleteListItemPlugin;
use reorder_button::ReorderButtonPlugin;
use reorder_list::ReorderListPlugin;

pub struct ListPlugin;

impl Plugin for ListPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ReorderButtonPlugin,
            AddListItemPlugin,
            ReorderListPlugin,
            DeleteListItemPlugin,
        ));
    }
}

#[derive(Component, Clone, Copy)]
pub struct List;

#[derive(Component, Clone, Copy)]
pub struct ListItem {
    /// This item's order in the list.
    index: usize,
}

impl ListItem {
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}
