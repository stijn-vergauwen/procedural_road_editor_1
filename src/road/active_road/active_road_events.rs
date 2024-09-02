pub mod new_road_component;
pub mod road_component_change;

use bevy::prelude::*;
use new_road_component::{NewRoadComponent, NewRoadComponentPlugin, NewRoadComponentRequest};
use road_component_change::{
    RoadComponentChange, RoadComponentChangePlugin, RoadComponentChangeRequest,
};

use crate::road::RoadData;

pub struct ActiveRoadEventsPlugin;

impl Plugin for ActiveRoadEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((NewRoadComponentPlugin, RoadComponentChangePlugin))
            .add_event::<OnActiveRoadChangeRequested>()
            .add_event::<OnActiveRoadChanged>();
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum RoadComponentField {
    Name,
    Width,
    Height,
    Color,
}

#[derive(Clone, PartialEq)]
pub enum ActiveRoadChangeRequest {
    AddRoadComponent(NewRoadComponentRequest),
    ChangeRoadComponent(RoadComponentChangeRequest),
    ReorderRoadComponent,
    DeleteRoadComponent,
}

#[derive(Event, Clone)]
pub struct OnActiveRoadChangeRequested {
    pub active_road_change_request: ActiveRoadChangeRequest,
}

impl OnActiveRoadChangeRequested {
    pub fn new(active_road_change_request: ActiveRoadChangeRequest) -> Self {
        Self {
            active_road_change_request,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ActiveRoadChange {
    RoadComponentAdded(NewRoadComponent),
    RoadComponentChanged(RoadComponentChange),
    RoadComponentReordered,
    RoadComponentDeleted,
}

#[derive(Event, Clone)]
pub struct OnActiveRoadChanged {
    pub active_road_change: ActiveRoadChange,
    pub previous_road_data: RoadData,
    pub new_road_data: RoadData,
}

impl OnActiveRoadChanged {
    pub fn new(
        active_road_change: ActiveRoadChange,
        previous_road_data: RoadData,
        new_road_data: RoadData,
    ) -> Self {
        Self {
            active_road_change,
            previous_road_data,
            new_road_data,
        }
    }
}
