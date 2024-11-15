use bevy::prelude::*;

use crate::road::road_node::{RequestedRoadNode, RoadNode};

#[derive(Clone, Copy, Debug)]
pub struct NearestRoadNode {
    pub position: Vec3,
    pub entity: Entity,
}

impl NearestRoadNode {
    pub fn find_from_point(
        road_node_query: &Query<(Entity, &Transform), With<RoadNode>>,
        point: Vec3,
        max_distance: f32,
    ) -> Option<Self> {
        let (node_entity, node_transform, node_distance) = road_node_query
            .iter()
            .map(|(node_entity, node_transform)| {
                (
                    node_entity,
                    node_transform,
                    node_transform.translation.distance(point),
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
