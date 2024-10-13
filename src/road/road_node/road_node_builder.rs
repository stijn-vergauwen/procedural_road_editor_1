use bevy::prelude::*;

use super::{RequestedRoadNode, RoadNode};

pub fn get_or_build_road_node(commands: &mut Commands, requested_node: RequestedRoadNode) -> Entity {
    if let Some(existing_entity) = requested_node.existing_node_entity {
        return existing_entity;
    }

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(requested_node.position)),
            RoadNode,
        ))
        .id()
}
