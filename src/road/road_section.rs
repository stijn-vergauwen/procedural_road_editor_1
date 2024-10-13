pub mod road_section_builder;

use bevy::prelude::*;
use road_section_builder::RoadSectionBuilderPlugin;

use super::road_node::RequestedRoadNode;

pub struct RoadSectionPlugin;

impl Plugin for RoadSectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoadSectionBuilderPlugin);
    }
}

// TODO: enum for straight or curved (each curve is its own section)

/// A section of 3D road connecting 2 RoadNode entities.
/// - Nodes describe start & end position while Sections describe how they're connected.
#[derive(Component, Debug, Clone, Copy)]
pub struct RoadSection {
    start_node: Entity,
    end_node: Entity,
}

impl RoadSection {
    pub fn new(start_node: Entity, end_node: Entity) -> Self {
        Self {
            start_node,
            end_node,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RequestedRoadSection {
    pub start: RequestedRoadNode,
    pub end: RequestedRoadNode,
}
