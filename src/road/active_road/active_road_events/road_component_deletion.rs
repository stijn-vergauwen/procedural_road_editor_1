use bevy::prelude::*;

use crate::{
    road::{
        active_road::{changed_component_indices::ChangedComponentIndices, ActiveRoad},
        road_data::RoadData,
    },
    ui::{
        list::list_events::list_item_deletion::{ListItemDeletion, OnListItemDeletionRequested},
        toolbar::RoadComponentsList,
    },
    utility::changed_value::ChangedValue,
    GameRunningSet,
};

pub struct RoadComponentDeletionPlugin;

impl Plugin for RoadComponentDeletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentDeletionRequested>()
            .add_event::<OnRoadComponentDeleted>()
            .add_systems(
                Update,
                handle_component_deletion_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event, Clone, PartialEq, Debug)]
pub struct OnRoadComponentDeletionRequested {
    pub index_to_delete: usize,
}

impl OnRoadComponentDeletionRequested {
    pub fn new(index_to_delete: usize) -> Self {
        Self { index_to_delete }
    }
}

#[derive(Event, Clone, PartialEq, Debug)]
pub struct OnRoadComponentDeleted {
    pub deleted_index: usize,
    pub changed_road_data: ChangedValue<RoadData>,
    pub changed_component_indices: ChangedComponentIndices,
}

impl OnRoadComponentDeleted {
    pub fn new(
        deleted_index: usize,
        changed_road_data: ChangedValue<RoadData>,
        changed_component_indices: ChangedComponentIndices,
    ) -> Self {
        Self {
            deleted_index,
            changed_road_data,
            changed_component_indices,
        }
    }

    pub fn previous_road_data(&self) -> &RoadData {
        &self.changed_road_data.previous_value
    }

    pub fn new_road_data(&self) -> &RoadData {
        &self.changed_road_data.new_value
    }
}

fn handle_component_deletion_requests(
    mut requests: EventReader<OnRoadComponentDeletionRequested>,
    mut on_deleted: EventWriter<OnRoadComponentDeleted>,
    mut on_list_item_deleted: EventWriter<OnListItemDeletionRequested>,
    mut active_road: ResMut<ActiveRoad>,
    road_components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for request in requests.read() {
        let previous_road_data = active_road.road_data().clone();

        let changed_component_indices = calculate_changed_component_indices(
            request.index_to_delete,
            previous_road_data.component_count(),
        );

        active_road.delete_road_component(request.index_to_delete);
        active_road.update_road_marking_positions(&previous_road_data, &changed_component_indices);

        let new_road_data = active_road.road_data().clone();

        on_deleted.send(OnRoadComponentDeleted::new(
            request.index_to_delete,
            ChangedValue::new(previous_road_data, new_road_data),
            changed_component_indices,
        ));

        if let Ok(road_components_list_entity) = road_components_list_query.get_single() {
            on_list_item_deleted.send(OnListItemDeletionRequested::new(ListItemDeletion::new(
                road_components_list_entity,
                request.index_to_delete,
            )));
        }
    }
}

fn calculate_changed_component_indices(
    index_to_delete: usize,
    previous_component_count: usize,
) -> ChangedComponentIndices {
    let mut changed_indices = Vec::new();

    changed_indices.push(ChangedValue::new(Some(index_to_delete), None));

    for index_to_shift in (index_to_delete + 1)..previous_component_count {
        changed_indices.push(ChangedValue::new(
            Some(index_to_shift),
            Some(index_to_shift - 1),
        ));
    }

    ChangedComponentIndices::new(changed_indices)
}
