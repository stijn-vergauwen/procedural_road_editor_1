use bevy::prelude::*;

use crate::GameRunningSet;

use super::{List, ListItem};

pub struct ReorderListPlugin;

impl Plugin for ReorderListPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnListReordered>().add_systems(
            Update,
            handle_reorder_events.in_set(GameRunningSet::UpdateEntities),
        );
    }
}

#[derive(Event)]
pub struct OnListReordered {
    list_entity: Entity,
    previous_index: usize,
    new_index: usize,
}

impl OnListReordered {
    pub fn new(list_entity: Entity, previous_index: usize, new_index: usize) -> Self {
        Self {
            list_entity,
            previous_index,
            new_index,
        }
    }

    pub fn list_entity(&self) -> Entity {
        self.list_entity
    }

    pub fn previous_index(&self) -> usize {
        self.previous_index
    }

    pub fn new_index(&self) -> usize {
        self.new_index
    }
}

fn handle_reorder_events(
    mut on_reordered: EventReader<OnListReordered>,
    mut list_children_query: Query<&mut Children, With<List>>,
    mut list_item_query: Query<&mut ListItem>,
) {
    for event in on_reordered.read() {
        let mut component_list_children = list_children_query.get_mut(event.list_entity()).unwrap();

        component_list_children.swap(event.previous_index(), event.new_index());

        for child in component_list_children.iter() {
            let mut component_item = list_item_query.get_mut(*child).unwrap();
            let item_index = component_item.index();

            if item_index == event.previous_index() {
                component_item.set_index(event.new_index());
            } else if item_index == event.new_index() {
                component_item.set_index(event.previous_index());
            }
        }
    }
}
