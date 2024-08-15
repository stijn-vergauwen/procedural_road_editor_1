use bevy::prelude::*;

use crate::{
    ui::list::{
        add_list_item::OnListItemAdded, reorder_list_item::OnListItemReordered, List, ListItem,
    },
    utility::{entity_is_descendant_of, partial::Partial},
    GameRunningSet,
};

use super::{ReorderButton, ReorderDirection};

pub struct ReorderButtonVisibilityPlugin;

impl Plugin for ReorderButtonVisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_reorder_buttons_on_add_event,
                update_reorder_buttons_on_reorder_event,
            )
                .after(GameRunningSet::UpdateEntities),
        );
    }
}

// TODO: refactor this module
// TODO: add OnListReplaced event (when all list items are replaced)
// TODO: update reorder button on list items set event

fn update_reorder_buttons_on_add_event(
    mut on_added: EventReader<OnListItemAdded>,
    mut reorder_button_query: Query<(Entity, &ReorderButton, &mut Visibility, &Partial)>,
    list_query: Query<&Children, With<List>>,
    list_item_query: Query<&ListItem>,
    parent_query: Query<&Parent>,
) {
    for event in on_added.read() {
        let list_children = list_query.get(event.list_entity()).unwrap();
        let list_length = list_children.len();

        update_visibility_of_reorder_buttons(
            &mut reorder_button_query,
            &parent_query,
            &list_item_query,
            event.list_entity(),
            list_length,
        );
    }
}

fn update_reorder_buttons_on_reorder_event(
    mut on_reordered: EventReader<OnListItemReordered>,
    mut reorder_button_query: Query<(Entity, &ReorderButton, &mut Visibility, &Partial)>,
    list_query: Query<&Children, With<List>>,
    list_item_query: Query<&ListItem>,
    parent_query: Query<&Parent>,
) {
    for event in on_reordered.read() {
        let list_children = list_query.get(event.list_entity()).unwrap();
        let list_length = list_children.len();

        update_visibility_of_reorder_buttons(
            &mut reorder_button_query,
            &parent_query,
            &list_item_query,
            event.list_entity(),
            list_length,
        );
    }
}

fn update_visibility_of_reorder_buttons(
    reorder_button_query: &mut Query<(Entity, &ReorderButton, &mut Visibility, &Partial)>,
    parent_query: &Query<&Parent>,
    list_item_query: &Query<&ListItem>,
    list_entity: Entity,
    list_length: usize,
) {
    // TODO: replace query params with "reorder_buttons", then filter on parent entity outside this fn
    for (_, reorder_button, mut button_visibility, reorder_button_partial) in reorder_button_query
        .iter_mut()
        .filter(|(button_entity, _, _, _)| {
            entity_is_descendant_of(parent_query, *button_entity, list_entity)
        })
    {
        let list_item = list_item_query
            .get(reorder_button_partial.main_entity())
            .unwrap();

        update_reorder_button_visibility(
            reorder_button,
            &mut button_visibility,
            list_length,
            list_item.index(),
        );
    }
}

fn update_reorder_button_visibility(
    reorder_button: &ReorderButton,
    button_visibility: &mut Visibility,
    list_length: usize,
    index: usize,
) {
    let target_visibility =
        calculate_target_visibility(reorder_button.direction(), index, list_length);

    if *button_visibility != target_visibility {
        *button_visibility = target_visibility;
    }
}

fn calculate_target_visibility(
    reorder_direction: ReorderDirection,
    index: usize,
    list_length: usize,
) -> Visibility {
    let should_be_visible = match reorder_direction {
        ReorderDirection::Next => index < list_length - 1,
        ReorderDirection::Previous => index > 0,
    };

    visibility_from_bool(should_be_visible)
}

fn visibility_from_bool(is_visible: bool) -> Visibility {
    match is_visible {
        true => Visibility::default(),
        false => Visibility::Hidden,
    }
}
