use bevy::prelude::*;

use crate::{
    road::{active_road::ActiveRoad, road_data::RoadData},
    ui::{
        list::list_events::list_reorder::{OnListReorderRequested, ReorderIndices},
        toolbar::RoadComponentsList,
    },
    GameRunningSet,
};

use super::ChangedRoadData;

pub struct RoadComponentReorderPlugin;

impl Plugin for RoadComponentReorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentReorderRequested>()
            .add_event::<OnRoadComponentReordered>()
            .add_systems(
                Update,
                handle_component_reorder_requests.in_set(GameRunningSet::HandleCommands),
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
    pub reorder: ReorderIndices,
    pub changed_road_data: ChangedRoadData,
}

impl OnRoadComponentReordered {
    pub fn new(reorder: ReorderIndices, changed_road_data: ChangedRoadData) -> Self {
        Self {
            reorder,
            changed_road_data,
        }
    }

    pub fn previous_road_data(&self) -> &RoadData {
        &self.changed_road_data.previous_road_data
    }

    pub fn new_road_data(&self) -> &RoadData {
        &self.changed_road_data.new_road_data
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
        let previous_road_data = active_road.road_data().clone();

        active_road.reorder_road_components(request.reorder);

        // TODO: this doesn't work, need to check switched indices
        active_road.update_road_marking_positions(&previous_road_data);

        let new_road_data = active_road.road_data().clone();

        on_reordered.send(OnRoadComponentReordered::new(
            request.reorder,
            ChangedRoadData::new(previous_road_data, new_road_data),
        ));

        if let Ok(road_components_list_entity) = road_components_list_query.get_single() {
            on_list_reorder.send(OnListReorderRequested::new(
                request.reorder,
                road_components_list_entity,
            ));
        }
    }
}
