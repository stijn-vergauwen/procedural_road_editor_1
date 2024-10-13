pub mod gizmos;
pub mod road_node_builder;

use bevy::prelude::*;
use gizmos::RoadNodeGizmosPlugin;

pub struct RoadNodePlugin;

impl Plugin for RoadNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoadNodeGizmosPlugin);
    }
}

/// RoadNodes are the points in a road network, they're connected by RoadSections.
/// - Nodes describe start & end position while Sections describe how they're connected.
#[derive(Component, Debug, Clone)]
pub struct RoadNode;

/// Describes a RoadNode that may or may not exist yet.
#[derive(Clone, Copy, Debug)]
pub struct RequestedRoadNode {
    pub position: Vec3,
    /// Entity of an existing RoadNode or None.
    pub existing_node_entity: Option<Entity>,
}

impl RequestedRoadNode {
    pub fn new(position: Vec3, existing_node_entity: Option<Entity>) -> Self {
        Self {
            position,
            existing_node_entity,
        }
    }
}
