use bevy::prelude::*;

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
