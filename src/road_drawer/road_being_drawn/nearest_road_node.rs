use bevy::prelude::*;

use crate::road::road_node::{RequestedRoadNode, RoadNode};

#[derive(Clone, Copy, Debug)]
pub struct NearestRoadNode {
    pub position: Vec3,
    pub entity: Entity,
}

impl NearestRoadNode {
    pub fn find_from_position(
        road_node_query: &Query<(Entity, &Transform), With<RoadNode>>,
        position: Vec3,
        max_distance: f32,
    ) -> Option<Self> {
        let (node_entity, node_transform, node_distance) = road_node_query
            .iter()
            .map(|(node_entity, node_transform)| {
                (
                    node_entity,
                    node_transform,
                    node_transform.translation.distance(position),
                )
            })
            .min_by(|(_, _, distance_a), (_, _, distance_b)| distance_a.total_cmp(&distance_b))?;

        (node_distance < max_distance).then_some(Self {
            position: node_transform.translation,
            entity: node_entity,
        })
    }

    pub fn to_requested_road_node(&self) -> RequestedRoadNode {
        RequestedRoadNode::new(self.position, Some(self.entity))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RoadNodeNearestToPosition {
    /// The original, unsnapped position.
    pub position: Vec3,
    pub nearest_road_node: Option<NearestRoadNode>,
}

impl RoadNodeNearestToPosition {
    pub fn new(position: Vec3, nearest_road_node: Option<NearestRoadNode>) -> Self {
        Self {
            position,
            nearest_road_node,
        }
    }

    /// Returns the position of nearest_road_node if Some, otherwise returns the position of self.
    pub fn snapped_position(&self) -> Vec3 {
        match self.nearest_road_node {
            Some(nearest_node) => nearest_node.position,
            None => self.position,
        }
    }

    pub fn to_requested_road_node(&self) -> RequestedRoadNode {
        match self.nearest_road_node {
            Some(nearest_node) => nearest_node.to_requested_road_node(),
            None => RequestedRoadNode::new(self.position, None),
        }
    }
}
