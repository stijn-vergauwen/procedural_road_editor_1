mod buttons;
mod toolbar;

use bevy::prelude::*;
use buttons::{ButtonsPlugin, OnReorderButtonPressed};
use toolbar::ToolbarPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ButtonsPlugin, ToolbarPlugin))
            .add_systems(Update, test_events);
    }
}

#[derive(Component, Clone, Copy)]
struct List;

#[derive(Component, Clone, Copy)]
struct ListItem {
    /// Reference to the List entity this item belongs to.
    list_entity: Entity,
    /// This item's order in the list.
    order: u8,
}

impl ListItem {
    fn new(list_entity: Entity, order: u8) -> Self {
        Self { list_entity, order }
    }
}

fn test_events(mut events: EventReader<OnReorderButtonPressed>) {
    for event in events.read() {
        println!("ReorderButtonPressed event!");
    }
}
