pub mod new_road_component;
pub mod road_component_change;
pub mod road_component_deletion;
pub mod road_component_reorder;

use bevy::prelude::*;
use new_road_component::{NewRoadComponent, NewRoadComponentPlugin};
use road_component_change::{RoadComponentChange, RoadComponentChangePlugin};
use road_component_deletion::{RoadComponentDeletion, RoadComponentDeletionPlugin};
use road_component_reorder::{RoadComponentReorder, RoadComponentReorderPlugin};

use crate::road::road_data::RoadData;

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

#[derive(Clone, PartialEq)]
pub enum ActiveRoadChange {
    AddRoadComponent(NewRoadComponent),
    ChangeRoadComponent(RoadComponentChange),
    ReorderRoadComponent(RoadComponentReorder),
    DeleteRoadComponent(RoadComponentDeletion),
}

#[derive(Clone)]
pub struct RoadDataChange {
    pub previous_road_data: RoadData,
    pub new_road_data: RoadData,
}

impl RoadDataChange {
    pub fn new(previous_road_data: RoadData, new_road_data: RoadData) -> Self {
        Self {
            previous_road_data,
            new_road_data,
        }
    }
}

#[derive(Event, Clone)]
pub struct OnActiveRoadChangeRequested {
    pub change_request: ActiveRoadChange,
}

impl OnActiveRoadChangeRequested {
    pub fn new(change_request: ActiveRoadChange) -> Self {
        Self { change_request }
    }
}

#[derive(Event, Clone)]
pub struct OnActiveRoadChanged {
    pub change: ActiveRoadChange,
    pub road_data: RoadDataChange,
}

impl OnActiveRoadChanged {
    pub fn new(change: ActiveRoadChange, road_data: RoadDataChange) -> Self {
        Self { change, road_data }
    }

    pub fn previous_road_data(&self) -> &RoadData {
        &self.road_data.previous_road_data
    }

    pub fn new_road_data(&self) -> &RoadData {
        &self.road_data.new_road_data
    }
}
