mod builder;

use bevy::prelude::*;

use crate::{ui::ListItem, utility::partial::Partial, GameRunningSet};
pub use builder::spawn_reorder_button;

pub struct ReorderButtonPlugin;

impl Plugin for ReorderButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnReorderButtonPressed>().add_systems(
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
    button_query: Query<(&ReorderButton, &Interaction, &Partial), Changed<Interaction>>,
    list_item_query: Query<&Partial, With<ListItem>>,
) {
    for (button, interaction, button_partial) in button_query.iter() {
        if *interaction == Interaction::Pressed {
            let list_partial = list_item_query.get(button_partial.main_entity()).unwrap();

            on_pressed.send(OnReorderButtonPressed::new(
                list_partial.main_entity(),
                button_partial.main_entity(),
                button.direction,
            ));
        }
    }
}
