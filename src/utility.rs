pub mod changed_value;
pub mod mesh_builder;
pub mod texture_builder;

use bevy::{
    ecs::query::{QueryData, QueryFilter},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shutdown_on_control_q);
    }
}

// Utility systems

fn shutdown_on_control_q(
    input: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut on_exit: EventWriter<AppExit>,
) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyQ) {
        let Ok(mut window) = window_query.get_single_mut() else {
            return;
        };

        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;

        on_exit.send(AppExit::Success);
    }
}

pub fn despawn_component_recursive<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Utility functions

pub fn add_rotations_as_eulers(rotation_a: Quat, rotation_b: Quat, euler: EulerRot) -> Quat {
    let euler_a = rotation_a.to_euler(euler);
    let euler_b = rotation_b.to_euler(euler);

    Quat::from_euler(
        euler,
        euler_a.0 + euler_b.0,
        euler_a.1 + euler_b.1,
        euler_a.2 + euler_b.2,
    )
}

pub fn entity_is_descendant_of(
    parent_query: &Query<&Parent>,
    entity: Entity,
    target: Entity,
) -> bool {
    parent_query
        .iter_ancestors(entity)
        .find(|ancestor| *ancestor == target)
        .is_some()
}

pub fn entity_is_ancestor_of(
    children_query: &Query<&Children>,
    entity: Entity,
    target: Entity,
) -> bool {
    children_query
        .iter_descendants(entity)
        .find(|descendant| *descendant == target)
        .is_some()
}

/// Returns the first entity in `entities_to_search` that is a ancestor of `start_entity`.
///
/// - In this case we have the child entity, and are searching for the first matching ancestor in the given search query.
pub fn find_ancestor_of_entity<'a, Data, Filter, Closure: Fn(&Data::Item<'a>) -> Entity>(
    start_entity: Entity,
    entities_to_search: &'a Query<Data, Filter>,
    get_entity_from_data: Closure,
    parent_query: &Query<&Parent>,
) -> Option<Data::Item<'a>>
where
    Data: QueryData<ReadOnly = Data>,
    Filter: QueryFilter,
{
    entities_to_search.iter().find(|ancestor| {
        entity_is_descendant_of(&parent_query, start_entity, get_entity_from_data(ancestor))
    })
}

/// Returns the first entity in `entities_to_search` that is a ancestor of `start_entity`.
///
/// - In this case we have the child entity, and are searching for the first matching ancestor in the given search query.
pub fn find_ancestor_of_entity_mut<'a, Data, Filter, Closure: Fn(&Data::Item<'a>) -> Entity>(
    start_entity: Entity,
    entities_to_search: &'a mut Query<Data, Filter>,
    get_entity_from_data: Closure,
    parent_query: &Query<&Parent>,
) -> Option<Data::Item<'a>>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    entities_to_search.iter_mut().find(|ancestor| {
        entity_is_descendant_of(&parent_query, start_entity, get_entity_from_data(ancestor))
    })
}

/// Returns the first entity in `entities_to_search` that is a descendant of `start_entity`.
///
/// - In this case we have the parent entity, and are searching for the first matching descendant in the given search query.
///
/// - This assumes that there will be at most 1 matching entity.
#[allow(unused)]
pub fn find_descendant_of_entity<'a, Data, Filter, Closure: Fn(&Data::Item<'a>) -> Entity>(
    start_entity: Entity,
    entities_to_search: &'a Query<Data, Filter>,
    get_entity_from_data: Closure,
    children_query: &Query<&Children>,
) -> Option<Data::Item<'a>>
where
    Data: QueryData<ReadOnly = Data>,
    Filter: QueryFilter,
{
    entities_to_search.iter().find(|descendant| {
        entity_is_ancestor_of(
            &children_query,
            start_entity,
            get_entity_from_data(descendant),
        )
    })
}

/// Returns the first entity in `entities_to_search` that is a descendant of `start_entity`.
///
/// - In this case we have the parent entity, and are searching for the first matching descendant in the given search query.
///
/// - This assumes that there will be at most 1 matching entity.
pub fn find_descendant_of_entity_mut<'a, Data, Filter, Closure: Fn(&Data::Item<'a>) -> Entity>(
    start_entity: Entity,
    entities_to_search: &'a mut Query<Data, Filter>,
    get_entity_from_data: Closure,
    children_query: &Query<&Children>,
) -> Option<Data::Item<'a>>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    entities_to_search.iter_mut().find(|descendant| {
        entity_is_ancestor_of(
            &children_query,
            start_entity,
            get_entity_from_data(descendant),
        )
    })
}

/// Returns all entities in `entities_to_search` that are descendants of `start_entity`.
///
/// - In this case we have the parent entity, and are searching for all matching descendants in the given search query.
pub fn filter_descendants_of_entity<'a, Data, Filter, Closure: Fn(&Data::Item<'a>) -> Entity>(
    start_entity: Entity,
    entities_to_search: &'a Query<Data, Filter>,
    get_entity_from_data: Closure,
    children_query: &Query<&Children>,
) -> Vec<Data::Item<'a>>
where
    Data: QueryData<ReadOnly = Data>,
    Filter: QueryFilter,
{
    entities_to_search
        .iter()
        .filter(|descendant| {
            entity_is_ancestor_of(
                &children_query,
                start_entity,
                get_entity_from_data(descendant),
            )
        })
        .collect()
}

/// Returns all entities in `entities_to_search` that are descendants of `start_entity`.
///
/// - In this case we have the parent entity, and are searching for all matching descendants in the given search query.
pub fn filter_descendants_of_entity_mut<'a, Data, Filter, Closure: Fn(&Data::Item<'a>) -> Entity>(
    start_entity: Entity,
    entities_to_search: &'a mut Query<Data, Filter>,
    get_entity_from_data: Closure,
    children_query: &Query<&Children>,
) -> Vec<Data::Item<'a>>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    entities_to_search
        .iter_mut()
        .filter(|descendant| {
            entity_is_ancestor_of(
                &children_query,
                start_entity,
                get_entity_from_data(descendant),
            )
        })
        .collect()
}
