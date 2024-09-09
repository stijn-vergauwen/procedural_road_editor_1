pub mod new_road_component;
pub mod road_component_change;
pub mod road_component_deletion;
pub mod road_component_reorder;

use bevy::prelude::*;
use new_road_component::NewRoadComponentPlugin;
use road_component_change::RoadComponentChangePlugin;
use road_component_deletion::RoadComponentDeletionPlugin;
use road_component_reorder::RoadComponentReorderPlugin;

use crate::road::road_data::RoadData;

pub struct ActiveRoadEventsPlugin;

impl Plugin for ActiveRoadEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnActiveRoadSet>().add_plugins((
            NewRoadComponentPlugin,
            RoadComponentChangePlugin,
            RoadComponentReorderPlugin,
            RoadComponentDeletionPlugin,
        ));
    }
}

#[derive(Event)]
pub struct OnActiveRoadSet {
    road_data: RoadData,
}

impl OnActiveRoadSet {
    pub fn new(road_data: RoadData) -> Self {
        Self { road_data }
    }

    pub fn road_data(&self) -> &RoadData {
        &self.road_data
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChangedRoadData {
    pub previous_road_data: RoadData,
    pub new_road_data: RoadData,
}

impl ChangedRoadData {
    pub fn new(previous_road_data: RoadData, new_road_data: RoadData) -> Self {
        Self {
            previous_road_data,
            new_road_data,
        }
    }
}
