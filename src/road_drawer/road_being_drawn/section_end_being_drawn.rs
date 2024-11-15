use bevy::prelude::*;

use crate::road::{road_node::RequestedRoadNode, road_section::RequestedRoadSectionEnd};

use super::nearest_road_node::NearestRoadNode;

#[derive(Clone, Copy, Debug)]
pub struct SectionEndBeingDrawn {
    /// The position of this end without snapping.
    pub position: Vec3,
    /// The outwards facing direction that this end looks towards.
    pub direction: Option<Dir3>,
    /// The nearest node to snap to.
    pub nearest_road_node: Option<NearestRoadNode>,
}

impl SectionEndBeingDrawn {
    pub fn to_requested_road_section_end(&self) -> RequestedRoadSectionEnd {
        let direction = self
            .direction
            .expect("Direction should be Some before converting to requested section");

        let road_node = match self.nearest_road_node {
            Some(nearest_node) => nearest_node.to_requested_road_node(),
            None => RequestedRoadNode::new(self.position, None),
        };

        RequestedRoadSectionEnd {
            road_node,
            direction,
        }
    }

    /// Returns the position of nearest_road_node if Some, otherwise returns the position of self.
    pub fn snapped_position(&self) -> Vec3 {
        match self.nearest_road_node {
            Some(nearest_node) => nearest_node.position,
            None => self.position,
        }
    }

    /// Returns this end's direction which faces outwards, or None.
    pub fn outwards_direction(&self) -> Option<Dir3> {
        self.direction
    }

    /// Returns this end's direction but flipped to face inwards, or None.
    pub fn inwards_direction(&self) -> Option<Dir3> {
        self.direction.map(|direction| -direction)
    }

    /// Returns a Transform with this end's position facing outwards, or None if no direction is set.
    #[expect(unused)]
    pub fn outwards_transform(&self) -> Option<Transform> {
        Some(self.get_transform_with_direction(self.outwards_direction()?))
    }

    /// Returns a Transform with this end's position facing inwards, or None if no direction is set.
    pub fn inwards_transform(&self) -> Option<Transform> {
        Some(self.get_transform_with_direction(self.inwards_direction()?))
    }

    /// Returns a Transform with this end's position and the given direction.
    pub fn get_transform_with_direction(&self, direction: Dir3) -> Transform {
        Transform::from_translation(self.position).looking_to(direction, Dir3::Y)
    }
}
