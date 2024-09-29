use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    road::{
        active_road::{changed_component_indices::ChangedComponentIndices, ActiveRoad},
        road_data::RoadData,
    },
    ui::{
        list::list_events::list_reorder::{OnListReorderRequested, ReorderIndices},
        toolbar::RoadComponentsList,
    },
    utility::changed_value::ChangedValue,
    GameRunningSet,
};

pub struct RoadComponentReorderPlugin;

impl Plugin for RoadComponentReorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentReorderRequested>()
            .add_event::<OnRoadComponentReordered>()
            .add_systems(
                Update,
                handle_component_reorder_requests
                    .in_set(GameRunningSet::HandleCommands)
                    .run_if(in_state(GameMode::RoadEditor)),
            );
    }
}

#[derive(Event, Clone, PartialEq, Debug)]
pub struct OnRoadComponentReorderRequested {
    pub reorder: ReorderIndices,
}

impl OnRoadComponentReorderRequested {
    pub fn new(reorder: ReorderIndices) -> Self {
        Self { reorder }
    }
}

#[derive(Event, Clone, PartialEq, Debug)]
pub struct OnRoadComponentReordered {
    pub changed_road_data: ChangedValue<RoadData>,
    pub changed_component_indices: ChangedComponentIndices,
}

impl OnRoadComponentReordered {
    pub fn new(
        changed_road_data: ChangedValue<RoadData>,
        changed_component_indices: ChangedComponentIndices,
    ) -> Self {
        Self {
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

fn handle_component_reorder_requests(
    mut requests: EventReader<OnRoadComponentReorderRequested>,
    mut on_reordered: EventWriter<OnRoadComponentReordered>,
    mut on_list_reorder: EventWriter<OnListReorderRequested>,
    mut active_road: ResMut<ActiveRoad>,
    road_components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for request in requests.read() {
        let changed_component_indices = ChangedComponentIndices::from_reorder(request.reorder);
        let previous_road_data = active_road.road_data().clone();

        active_road.reorder_road_components(request.reorder);

        active_road.update_road_marking_positions(&previous_road_data, &changed_component_indices);

        let new_road_data = active_road.road_data().clone();

        on_reordered.send(OnRoadComponentReordered::new(
            ChangedValue::new(previous_road_data, new_road_data),
            changed_component_indices,
        ));

        if let Ok(road_components_list_entity) = road_components_list_query.get_single() {
            on_list_reorder.send(OnListReorderRequested::new(
                request.reorder,
                road_components_list_entity,
            ));
        }
    }
}
