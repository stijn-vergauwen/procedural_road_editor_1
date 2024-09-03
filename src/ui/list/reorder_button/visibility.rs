use bevy::prelude::*;

use crate::{
    ui::list::{add_list_item::OnListItemAdded, reorder_list::OnListReordered, List, ListItem},
    utility::{filter_descendants_of_entity_mut, find_ancestor_of_entity},
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
                .in_set(GameRunningSet::UpdateEntities),
        );
    }
}

// TODO: refactor this module

/*
    TODO: I have a question related to event architecture:
    Logic like recalculating these reorder buttons or redrawing the preview road should respond to more than 1 event in the same way,
    The nice thing about events is that many systems can respond to 1 event, but in this case 1 system should respond to many events.
    It's the other way around, What is a clean way to handle these cases? This is something I'm struggling to find a good answer for atm.
*/

fn update_reorder_buttons_on_add_event(
    mut on_added: EventReader<OnListItemAdded>,
    mut reorder_button_query: Query<(Entity, &ReorderButton, &mut Visibility)>,
    list_query: Query<&Children, With<List>>,
    list_item_query: Query<(Entity, &ListItem)>,
    parent_query: Query<&Parent>,
    children_query: Query<&Children>,
) {
    for event in on_added.read() {
        let list_children = list_query.get(event.list_entity()).unwrap();
        let list_length = list_children.len();

        update_visibility_of_reorder_buttons(
            &mut reorder_button_query,
            &parent_query,
            &children_query,
            &list_item_query,
            event.list_entity(),
            list_length,
        );
    }
}

fn update_reorder_buttons_on_reorder_event(
    mut on_reordered: EventReader<OnListReordered>,
    mut reorder_button_query: Query<(Entity, &ReorderButton, &mut Visibility)>,
    list_query: Query<&Children, With<List>>,
    list_item_query: Query<(Entity, &ListItem)>,
    parent_query: Query<&Parent>,
    children_query: Query<&Children>,
) {
    for event in on_reordered.read() {
        let list_children = list_query.get(event.list_entity()).unwrap();
        let list_length = list_children.len();

        update_visibility_of_reorder_buttons(
            &mut reorder_button_query,
            &parent_query,
            &children_query,
            &list_item_query,
            event.list_entity(),
            list_length,
        );
    }
}

fn update_visibility_of_reorder_buttons(
    reorder_button_query: &mut Query<(Entity, &ReorderButton, &mut Visibility)>,
    parent_query: &Query<&Parent>,
    children_query: &Query<&Children>,
    list_item_query: &Query<(Entity, &ListItem)>,
    list_entity: Entity,
    list_length: usize,
) {
    // TODO: replace query params with "reorder_buttons", then filter on parent entity outside this fn
    for (reorder_button_entity, reorder_button, mut button_visibility) in
        filter_descendants_of_entity_mut(
            list_entity,
            reorder_button_query,
            |item| item.0,
            children_query,
        )
    {
        let (_, list_item) = find_ancestor_of_entity(
            reorder_button_entity,
            list_item_query,
            |item| item.0,
            parent_query,
        )
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
