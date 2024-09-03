use bevy::prelude::*;

use crate::{
    road::ActiveRoad,
    ui::{
        list::reorder_list::{OnListReorderRequested, ReorderIndices},
        toolbar::RoadComponentsList,
    },
    GameRunningSet,
};

use super::{ActiveRoadChange, OnActiveRoadChangeRequested, OnActiveRoadChanged, RoadDataChange};

pub struct RoadComponentReorderPlugin;

impl Plugin for RoadComponentReorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_component_reorder_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct RoadComponentReorder {
    pub reorder: ReorderIndices,
}

impl RoadComponentReorder {
    pub fn new(reorder: ReorderIndices) -> Self {
        Self { reorder }
    }
}

fn handle_component_reorder_requests(
    mut requests: EventReader<OnActiveRoadChangeRequested>,
    mut on_changed: EventWriter<OnActiveRoadChanged>,
    mut on_list_reorder: EventWriter<OnListReorderRequested>,
    mut active_road: ResMut<ActiveRoad>,
    road_components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for request in requests.read() {
        let reorder = match &request.change_request {
            ActiveRoadChange::ReorderRoadComponent(request) => request.reorder,
            _ => continue,
        };

        let previous_road_data = active_road.road_data().clone();

        active_road.reorder_road_components(reorder);

        let new_road_data = active_road.road_data().clone();

        on_changed.send(OnActiveRoadChanged::new(
            request.change_request.clone(),
            RoadDataChange::new(previous_road_data, new_road_data),
        ));

        if let Ok(road_components_list_entity) = road_components_list_query.get_single() {
            on_list_reorder.send(OnListReorderRequested::new(
                reorder,
                road_components_list_entity,
            ));
        }
    }
}
