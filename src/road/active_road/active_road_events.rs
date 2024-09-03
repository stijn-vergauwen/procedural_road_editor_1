pub mod new_road_component;
pub mod road_component_change;
pub mod road_component_deletion;
pub mod road_component_reorder;

use bevy::prelude::*;
use new_road_component::{NewRoadComponent, NewRoadComponentPlugin, NewRoadComponentRequest};
use road_component_change::{
    RoadComponentChange, RoadComponentChangePlugin, RoadComponentChangeRequest,
};
use road_component_deletion::{RoadComponentDeletion, RoadComponentDeletionPlugin};
use road_component_reorder::{RoadComponentReorder, RoadComponentReorderPlugin};

use crate::road::RoadData;

pub struct ActiveRoadEventsPlugin;

impl Plugin for ActiveRoadEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            NewRoadComponentPlugin,
            RoadComponentChangePlugin,
            RoadComponentReorderPlugin,
            RoadComponentDeletionPlugin,
        ))
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
    ReorderRoadComponent(RoadComponentReorder),
    DeleteRoadComponent(RoadComponentDeletion),
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

// TODO: delete this struct, use ActiveRoadChangeRequest for both request and event. This moves the code more to "single source of truth"
#[derive(Clone, PartialEq)]
pub enum ActiveRoadChange {
    RoadComponentAdded(NewRoadComponent),
    RoadComponentChanged(RoadComponentChange),
    RoadComponentReordered(RoadComponentReorder),
    RoadComponentDeleted(RoadComponentDeletion),
}

#[derive(Event, Clone)]
pub struct OnActiveRoadChanged {
    pub active_road_change: ActiveRoadChange,
    #[allow(unused)]
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
