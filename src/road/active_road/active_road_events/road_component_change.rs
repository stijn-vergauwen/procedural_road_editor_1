use bevy::prelude::*;

use crate::{road::ActiveRoad, GameRunningSet};

use super::{ActiveRoadChange, OnActiveRoadChangeRequested, OnActiveRoadChanged, RoadDataChange};

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
pub struct RoadComponentChange {
    pub field: RoadComponentFieldChange,
    pub road_component_index: usize,
}

impl RoadComponentChange {
    pub fn new(field: RoadComponentFieldChange, road_component_index: usize) -> Self {
        Self {
            field,
            road_component_index,
        }
    }
}

fn handle_component_change_requests(
    mut requests: EventReader<OnActiveRoadChangeRequested>,
    mut on_changed: EventWriter<OnActiveRoadChanged>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests.read() {
        let change_request = match &request.change_request {
            ActiveRoadChange::ChangeRoadComponent(request) => request,
            _ => continue,
        };

        let previous_road_data = active_road.road_data().clone();

        active_road.change_road_component_at_index(
            change_request.road_component_index,
            change_request.field.clone(),
        );

        let new_road_data = active_road.road_data().clone();

        on_changed.send(OnActiveRoadChanged::new(
            request.change_request.clone(),
            RoadDataChange::new(previous_road_data, new_road_data),
        ));
    }
}
