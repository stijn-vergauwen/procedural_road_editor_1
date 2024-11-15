use bevy::prelude::*;

use crate::road::road_section::RequestedRoadSectionEnd;

use super::nearest_road_node::{NearestRoadNode, RoadNodeNearestToPosition};

#[derive(Clone, Copy, Debug)]
pub struct SectionEndBeingDrawn {
    pub road_node_nearest_to_position: RoadNodeNearestToPosition,
}

impl SectionEndBeingDrawn {
    pub fn new(position: Vec3, nearest_road_node: Option<NearestRoadNode>) -> Self {
        Self {
            road_node_nearest_to_position: RoadNodeNearestToPosition::new(
                position,
                nearest_road_node,
            ),
        }
    }

    pub fn to_requested_road_section_end(&self, direction: Dir3) -> RequestedRoadSectionEnd {
        let road_node = self.road_node_nearest_to_position.to_requested_road_node();

        RequestedRoadSectionEnd {
            road_node,
            direction,
        }
    }

    /// Returns the position of nearest_road_node if Some, otherwise returns the position of self.
    pub fn snapped_position(&self) -> Vec3 {
        self.road_node_nearest_to_position.snapped_position()
    }
}
