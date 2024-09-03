use bevy::prelude::*;

use crate::GameRunningSet;

use super::ListItem;

pub struct DeleteListItemPlugin;

impl Plugin for DeleteListItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnListItemDeletionRequested>()
            .add_event::<OnListItemDeleted>()
            .add_systems(
                Update,
                handle_deletion_events.in_set(GameRunningSet::DespawnEntities),
            );
    }
}

#[derive(Clone, Copy)]
pub struct ListItemDeletion {
    pub list_entity: Entity,
    pub index_to_delete: usize,
}

impl ListItemDeletion {
    pub fn new(list_entity: Entity, index_to_delete: usize) -> Self {
        Self {
            list_entity,
            index_to_delete,
        }
    }
}

#[derive(Event, Clone, Copy)]
pub struct OnListItemDeletionRequested {
    deletion: ListItemDeletion,
}

impl OnListItemDeletionRequested {
    pub fn new(deletion: ListItemDeletion) -> Self {
        Self { deletion }
    }

    pub fn list_entity(&self) -> Entity {
        self.deletion.list_entity
    }

    pub fn index_to_delete(&self) -> usize {
        self.deletion.index_to_delete
    }
}

#[derive(Event, Clone, Copy)]
pub struct OnListItemDeleted {
    deletion: ListItemDeletion,
}

impl OnListItemDeleted {
    pub fn new(deletion: ListItemDeletion) -> Self {
        Self { deletion }
    }

    pub fn list_entity(&self) -> Entity {
        self.deletion.list_entity
    }

    pub fn deleted_index(&self) -> usize {
        self.deletion.index_to_delete
    }
}

fn handle_deletion_events(
    mut requests: EventReader<OnListItemDeletionRequested>,
    mut on_deleted: EventWriter<OnListItemDeleted>,
    mut commands: Commands,
    mut list_item_query: Query<(Entity, &mut ListItem)>,
) {
    for request in requests.read() {
        for (list_item_entity, mut list_item) in list_item_query.iter_mut() {
            if list_item.index() == request.index_to_delete() {
                delete_list_item(&mut commands, list_item_entity);
            } else if list_item.index() > request.index_to_delete() {
                decrement_list_item_index(&mut list_item);
            }
        }

        on_deleted.send(OnListItemDeleted::new(request.deletion));
    }
}

fn delete_list_item(commands: &mut Commands, list_item_entity: Entity) {
    let mut list_item_commands = commands.entity(list_item_entity);

    list_item_commands.remove_parent();
    list_item_commands.despawn_recursive();
}

fn decrement_list_item_index(list_item: &mut ListItem) {
    let new_index = list_item.index() - 1;
    list_item.set_index(new_index);
}
