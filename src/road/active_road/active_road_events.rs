pub mod new_road_component;
pub mod road_component_change;
pub mod road_component_deletion;
pub mod road_component_reorder;

use bevy::prelude::*;
use new_road_component::{NewRoadComponent, NewRoadComponentPlugin};
use road_component_change::{RoadComponentChange, RoadComponentChangePlugin};
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
pub enum ActiveRoadChange {
    AddRoadComponent(NewRoadComponent),
    ChangeRoadComponent(RoadComponentChange),
    ReorderRoadComponent(RoadComponentReorder),
    DeleteRoadComponent(RoadComponentDeletion),
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
    #[allow(unused)]
    pub previous_road_data: RoadData,
    pub new_road_data: RoadData,
}

impl OnActiveRoadChanged {
    pub fn new(
        change: ActiveRoadChange,
        previous_road_data: RoadData,
        new_road_data: RoadData,
    ) -> Self {
        Self {
            change,
            previous_road_data,
            new_road_data,
        }
    }
}
