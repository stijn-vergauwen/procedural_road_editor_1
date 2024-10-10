use bevy::prelude::*;

use crate::{
    ui::list::{List, ListItem},
    GameRunningSet,
};

pub struct ListReorderPlugin;

impl Plugin for ListReorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnListReorderRequested>()
            .add_event::<OnListReordered>()
            .add_systems(
                Update,
                handle_reorder_events.before(GameRunningSet::UpdateEntities),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReorderIndices {
    pub previous_index: usize,
    pub new_index: usize,
}

impl ReorderIndices {
    pub fn new(previous_index: usize, new_index: usize) -> Self {
        Self {
            previous_index,
            new_index,
        }
    }
}

#[derive(Event, Clone, Copy)]
pub struct OnListReorderRequested {
    pub reorder: ReorderIndices,
    pub list_entity: Entity,
}

impl OnListReorderRequested {
    pub fn new(reorder: ReorderIndices, list_entity: Entity) -> Self {
        Self {
            reorder,
            list_entity,
        }
    }
}

#[derive(Event, Clone, Copy)]
pub struct OnListReordered {
    pub reorder: ReorderIndices,
    pub list_entity: Entity,

    /// Entity of the list item that moved from previous index to new index.
    #[expect(unused)]
    pub target_list_item_entity: Entity,

    /// Entity of the list item that moved from new index to previous index.
    #[expect(unused)]
    pub swapped_list_item_entity: Entity,
}

impl OnListReordered {
    pub fn new(
        reorder: ReorderIndices,
        list_entity: Entity,
        target_list_item_entity: Entity,
        swapped_list_item_entity: Entity,
    ) -> Self {
        Self {
            reorder,
            list_entity,
            target_list_item_entity,
            swapped_list_item_entity,
        }
    }

    pub fn list_entity(&self) -> Entity {
        self.list_entity
    }

    #[expect(unused)]
    pub fn previous_index(&self) -> usize {
        self.reorder.previous_index
    }

    #[expect(unused)]
    pub fn new_index(&self) -> usize {
        self.reorder.new_index
    }
}

fn handle_reorder_events(
    mut requests: EventReader<OnListReorderRequested>,
    mut on_reordered: EventWriter<OnListReordered>,
    mut list_children_query: Query<&mut Children, With<List>>,
    mut list_item_query: Query<&mut ListItem>,
) {
    for request in requests.read() {
        let mut children_of_list = list_children_query.get_mut(request.list_entity).unwrap();
        let previous_index = request.reorder.previous_index;
        let new_index = request.reorder.new_index;

        children_of_list.swap(previous_index, new_index);

        let mut target_list_item_entity: Option<Entity> = None;
        let mut swapped_list_item_entity: Option<Entity> = None;

        for child in children_of_list.iter() {
            let mut component_item = list_item_query.get_mut(*child).unwrap();
            let item_index = component_item.index();

            if item_index == previous_index {
                component_item.set_index(new_index);

                target_list_item_entity = Some(*child);
            } else if item_index == new_index {
                component_item.set_index(previous_index);

                swapped_list_item_entity = Some(*child);
            }
        }

        on_reordered.send(OnListReordered::new(
            request.reorder,
            request.list_entity,
            target_list_item_entity.unwrap(),
            swapped_list_item_entity.unwrap(),
        ));
    }
}
