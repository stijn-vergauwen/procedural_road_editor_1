use bevy::prelude::*;

pub struct NewListItemPlugin;

impl Plugin for NewListItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnListItemAdded>();
    }
}

#[derive(Event)]
pub struct OnListItemAdded {
    list_entity: Entity,
}

impl OnListItemAdded {
    pub fn new(list_entity: Entity) -> Self {
        Self { list_entity }
    }

    pub fn list_entity(&self) -> Entity {
        self.list_entity
    }
}
