use bevy::prelude::*;

use crate::{
    road::{active_road::ActiveRoad, road_data::RoadData},
    GameRunningSet,
};

use super::ChangedRoadData;

pub struct RoadComponentChangePlugin;

impl Plugin for RoadComponentChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentChangeRequested>()
            .add_event::<OnRoadComponentChanged>()
            .add_systems(
                Update,
                handle_component_change_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Component, Clone, PartialEq, Debug)]
pub enum RoadComponentFieldChange {
    Name(String),
    Width(f32),
    Height(f32),
    Color(Color),
}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Event, Clone, PartialEq, Debug)]
pub struct OnRoadComponentChangeRequested {
    pub requested_change: RoadComponentChange,
}

impl OnRoadComponentChangeRequested {
    pub fn new(requested_change: RoadComponentChange) -> Self {
        Self { requested_change }
    }
}

#[derive(Event, Clone, PartialEq, Debug)]
pub struct OnRoadComponentChanged {
    pub change: RoadComponentChange,
    pub changed_road_data: ChangedRoadData,
}

impl OnRoadComponentChanged {
    pub fn new(change: RoadComponentChange, changed_road_data: ChangedRoadData) -> Self {
        Self {
            change,
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

fn handle_component_change_requests(
    mut requests: EventReader<OnRoadComponentChangeRequested>,
    mut on_changed: EventWriter<OnRoadComponentChanged>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests.read() {
        let previous_road_data = active_road.road_data().clone();

        active_road.change_road_component_at_index(
            request.requested_change.road_component_index,
            request.requested_change.field.clone(),
        );

        let new_road_data = active_road.road_data().clone();

        on_changed.send(OnRoadComponentChanged::new(
            request.requested_change.clone(),
            ChangedRoadData::new(previous_road_data, new_road_data),
        ));
    }
}
