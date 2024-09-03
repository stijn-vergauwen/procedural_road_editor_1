use bevy::prelude::*;

use crate::{
    road::{ActiveRoad, RoadComponent},
    GameRunningSet,
};

use super::{
    ActiveRoadChange, ActiveRoadChangeRequest, OnActiveRoadChangeRequested, OnActiveRoadChanged,
};

pub struct RoadComponentChangePlugin;

impl Plugin for RoadComponentChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_component_change_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Component, Clone, PartialEq)]
pub enum RoadComponentFieldChange {
    Name(String),
    Width(f32),
    Height(f32),
    Color(Color),
}

#[derive(Clone, PartialEq)]
pub struct RoadComponentChangeRequest {
    field_to_change: RoadComponentFieldChange,
    road_component_index: usize,
}

impl RoadComponentChangeRequest {
    pub fn new(field_to_change: RoadComponentFieldChange, road_component_index: usize) -> Self {
        Self {
            field_to_change,
            road_component_index,
        }
    }
}

// TODO: delete this struct, use RoadComponentChangeRequest for both request and event. This moves the code more to "single source of truth"
#[derive(Clone, PartialEq)]
pub struct RoadComponentChange {
    pub changed_field: RoadComponentFieldChange,
    pub new_road_component: RoadComponent,
    pub road_component_index: usize,
}

impl RoadComponentChange {
    pub fn new(
        changed_field: RoadComponentFieldChange,
        new_road_component: RoadComponent,
        road_component_index: usize,
    ) -> Self {
        Self {
            changed_field,
            new_road_component,
            road_component_index,
        }
    }
}

fn handle_component_change_requests(
    mut requests: EventReader<OnActiveRoadChangeRequested>,
    mut on_changed: EventWriter<OnActiveRoadChanged>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests
        .read()
        .filter_map(|request| match &request.active_road_change_request {
            ActiveRoadChangeRequest::ChangeRoadComponent(request) => Some(request),
            _ => None,
        })
    {
        let previous_road_data = active_road.road_data().clone();

        let new_road_component = active_road.change_road_component_at_index(
            request.road_component_index,
            request.field_to_change.clone(),
        );

        let new_road_data = active_road.road_data().clone();

        on_changed.send(OnActiveRoadChanged::new(
            ActiveRoadChange::RoadComponentChanged(RoadComponentChange::new(
                request.field_to_change.clone(),
                new_road_component,
                request.road_component_index,
            )),
            previous_road_data,
            new_road_data,
        ));
    }
}
