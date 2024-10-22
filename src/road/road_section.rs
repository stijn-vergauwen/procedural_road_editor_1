pub mod gizmos;
pub mod road_section_builder;

use bevy::prelude::*;
use gizmos::RoadSectionGizmosPlugin;
use road_section_builder::RoadSectionBuilderPlugin;

use super::{road_data::RoadData, road_node::RequestedRoadNode};

pub struct RoadSectionPlugin;

impl Plugin for RoadSectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RoadSectionBuilderPlugin, RoadSectionGizmosPlugin));
    }
}

// TODO: enum for straight or curved (each curve is its own section)

/// A section of 3D road connecting 2 RoadNode entities.
/// - Nodes describe start & end position while Sections describe how they're connected.
#[derive(Component, Debug, Clone)]
pub struct RoadSection {
    pub start_node: Entity,
    pub end_node: Entity,
    pub road_design: RoadData,
}

impl RoadSection {
    pub fn new(start_node: Entity, end_node: Entity, road_design: RoadData) -> Self {
        Self { start_node, end_node, road_design }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RequestedRoadSection {
    pub start: RequestedRoadNode,
    pub end: RequestedRoadNode,
}

fn calculate_road_section_transform(
    start_node_position: Vec3,
    end_node_position: Vec3,
) -> Transform {
    let delta_position = end_node_position - start_node_position;
    let section_position = start_node_position + delta_position / 2.0;
    let section_rotation = Transform::from_translation(start_node_position)
        .looking_at(end_node_position, Dir3::Y)
        .rotation;

    Transform {
        translation: section_position,
        rotation: section_rotation,
        ..default()
    }
}

fn calculate_road_section_size(
    road_design: &RoadData,
    start_node_position: Vec3,
    end_node_position: Vec3,
) -> Vec3 {
    road_design
        .total_size()
        .extend(start_node_position.distance(end_node_position))
}