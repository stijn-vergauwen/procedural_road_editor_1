mod builder;
mod visibility;

use bevy::prelude::*;
use visibility::ReorderButtonVisibilityPlugin;

use crate::{ui::list::ListItem, utility::find_ancestor_of_entity, GameRunningSet};
pub use builder::spawn_reorder_button;

use super::List;

pub struct ReorderButtonPlugin;

impl Plugin for ReorderButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReorderButtonVisibilityPlugin)
            .add_event::<OnReorderButtonPressed>()
            .add_systems(
                Update,
                send_reorder_button_pressed_events.in_set(GameRunningSet::GetUserInput),
            );
    }
}

#[derive(Event)]
pub struct OnReorderButtonPressed {
    list_entity: Entity,
    list_item_entity: Entity,
    direction: ReorderDirection,
}

impl OnReorderButtonPressed {
    pub fn new(list_entity: Entity, list_item_entity: Entity, direction: ReorderDirection) -> Self {
        Self {
            list_entity,
            list_item_entity,
            direction,
        }
    }

    pub fn list_entity(&self) -> Entity {
        self.list_entity
    }

    pub fn list_item_entity(&self) -> Entity {
        self.list_item_entity
    }

    pub fn direction(&self) -> ReorderDirection {
        self.direction
    }
}

#[derive(Component)]
pub struct ReorderButton {
    direction: ReorderDirection,
}

impl ReorderButton {
    pub fn direction(&self) -> ReorderDirection {
        self.direction
    }
}

#[derive(Clone, Copy)]
pub enum ReorderDirection {
    Next,
    Previous,
}

fn send_reorder_button_pressed_events(
    mut on_pressed: EventWriter<OnReorderButtonPressed>,
    button_query: Query<(Entity, &ReorderButton, &Interaction), Changed<Interaction>>,
    list_item_query: Query<Entity, With<ListItem>>,
    list_query: Query<Entity, With<List>>,
    parent_query: Query<&Parent>,
) {
    for (button_entity, button, _) in button_query
        .iter()
        .filter(|(_, _, interaction)| **interaction == Interaction::Pressed)
    {
        let list_item_entity =
            find_ancestor_of_entity(button_entity, &list_item_query, |item| *item, &parent_query)
                .unwrap();

        let list_entity =
            find_ancestor_of_entity(list_item_entity, &list_query, |item| *item, &parent_query)
                .unwrap();

        on_pressed.send(OnReorderButtonPressed::new(
            list_entity,
            list_item_entity,
            button.direction,
        ));
    }
}
