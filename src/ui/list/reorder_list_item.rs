use bevy::prelude::*;

pub struct ReorderListItemPlugin;

impl Plugin for ReorderListItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnListItemReordered>();
    }
}

#[derive(Event)]
pub struct OnListItemReordered {
    list_entity: Entity,
}

impl OnListItemReordered {
    pub fn new(list_entity: Entity) -> Self {
        Self { list_entity }
    }

    pub fn list_entity(&self) -> Entity {
        self.list_entity
    }
}
