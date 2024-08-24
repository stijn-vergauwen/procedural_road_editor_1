use bevy::prelude::*;

use crate::GameRunningSet;

use super::ListItem;

pub struct DeleteListItemPlugin;

impl Plugin for DeleteListItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnListItemDeleted>().add_systems(
            Update,
            handle_deletion_events.in_set(GameRunningSet::DespawnEntities),
        );
    }
}

#[derive(Event)]
pub struct OnListItemDeleted {
    list_entity: Entity,
    list_item_entity: Entity,
    index: usize,
}

impl OnListItemDeleted {
    pub fn new(list_entity: Entity, list_item_entity: Entity, index: usize) -> Self {
        Self {
            list_entity,
            list_item_entity,
            index,
        }
    }

    pub fn list_entity(&self) -> Entity {
        self.list_entity
    }

    pub fn list_item_entity(&self) -> Entity {
        self.list_item_entity
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

fn handle_deletion_events(
    mut on_deleted: EventReader<OnListItemDeleted>,
    mut commands: Commands,
    mut list_item_query: Query<(Entity, &mut ListItem)>,
) {
    for event in on_deleted.read() {
        let mut list_item_commands = commands.entity(event.list_item_entity());

        list_item_commands.remove_parent();
        list_item_commands.despawn_recursive();

        for (_, mut list_item) in
            list_item_query
                .iter_mut()
                .filter(|(list_item_entity, list_item)| {
                    *list_item_entity == event.list_item_entity()
                        && list_item.index() > event.index()
                })
        {
            let new_index = list_item.index() - 1;
            list_item.set_index(new_index);
        }
    }
}
