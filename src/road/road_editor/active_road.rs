use bevy::prelude::*;

use crate::road::road_data::RoadData;

#[derive(Component)]
pub struct ActiveRoad;

#[derive(Event)]
pub struct OnActiveRoadModified {
    road: RoadData,
}

impl OnActiveRoadModified {
    pub fn new(road: RoadData) -> Self {
        Self { road }
    }

    pub fn road(&self) -> &RoadData {
        &self.road
    }
}
