use bevy::{
    prelude::*,
    ui::{FocusPolicy, RelativeCursorPosition},
};

use crate::GameRunningSet;

const PRINT_MESSAGE_WHEN_VALUE_CHANGED: bool = false;

pub struct MouseOnUiPlugin;

impl Plugin for MouseOnUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseOnUi>().add_systems(
            Update,
            (
                update_mouse_on_ui.in_set(GameRunningSet::FetchData),
                add_relative_cursor_position_component_to_node_entities
                    .in_set(GameRunningSet::UpdateEntities),
            ),
        );
    }
}

#[derive(Resource, Debug, Default)]
pub struct MouseOnUi {
    is_on_ui: bool,
}

impl MouseOnUi {
    pub fn is_on_ui(&self) -> bool {
        self.is_on_ui
    }
}

fn update_mouse_on_ui(
    mut mouse_on_ui: ResMut<MouseOnUi>,
    node_query: Query<(&RelativeCursorPosition, &FocusPolicy, &ViewVisibility)>,
) {
    let mouse_is_on_ui =
        node_query
            .iter()
            .any(|(relative_position, focus_policy, view_visibility)| {
                *focus_policy == FocusPolicy::Block
                    && view_visibility.get()
                    && relative_position.mouse_over()
            });

    if mouse_is_on_ui != mouse_on_ui.is_on_ui {
        mouse_on_ui.is_on_ui = mouse_is_on_ui;

        if PRINT_MESSAGE_WHEN_VALUE_CHANGED {
            println!(
                "Mouse is now{} on UI.",
                if mouse_on_ui.is_on_ui { "" } else { " not" }
            );
        }
    }
}

fn add_relative_cursor_position_component_to_node_entities(
    mut commands: Commands,
    node_query: Query<Entity, (With<Node>, Without<RelativeCursorPosition>)>,
) {
    for node_entity in node_query.iter() {
        commands
            .entity(node_entity)
            .insert(RelativeCursorPosition::default());
    }
}
