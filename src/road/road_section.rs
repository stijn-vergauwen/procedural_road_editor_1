pub mod gizmos;
pub mod road_section_builder;

use bevy::prelude::*;
use gizmos::RoadSectionGizmosPlugin;
use road_section_builder::RoadSectionBuilderPlugin;

use crate::utility::circular_arc::CircularArc;

use super::{road_data::RoadData, road_node::RequestedRoadNode};

pub struct RoadSectionPlugin;

impl Plugin for RoadSectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RoadSectionBuilderPlugin, RoadSectionGizmosPlugin));
    }
}

// TODO: add RoadSectionDirection enum with values Normal & Reversed, this describes which road-end should be seen as "start" and which as "end"

/// A section of 3D road connecting 2 RoadNode entities.
/// - Nodes describe points while Sections describe how they're connected.
#[derive(Component, Debug, Clone)]
pub struct RoadSection {
    pub ends: [RoadSectionEnd; 2],
    pub variant: RoadSectionVariant,
    pub road_design: RoadData,
}

impl RoadSection {
    pub fn new(
        ends: [RoadSectionEnd; 2],
        variant: RoadSectionVariant,
        road_design: RoadData,
    ) -> Self {
        Self {
            ends,
            variant,
            road_design,
        }
    }

    /// Returns the RoadSectionEnd that is considered the "start" of this section.
    pub fn start(&self) -> RoadSectionEnd {
        self.ends[0]
    }

    /// Returns the RoadSectionEnd that is considered the "end" of this section.
    pub fn end(&self) -> RoadSectionEnd {
        self.ends[1]
    }
}

/// The ends of a `RoadSection`
#[derive(Clone, Copy, Debug)]
pub struct RoadSectionEnd {
    /// The entity of the road node this end is connected to.
    pub road_node_entity: Entity,

    // TODO: can't this direction just be calculated when needed? Removing this would remove a lot of code in the drawer that only exists to calculate the direction
    /// The outwards facing direction that this end looks towards.
    #[expect(unused)]
    pub direction: Dir3,
}

impl RoadSectionEnd {
    pub fn new(road_node_entity: Entity, direction: Dir3) -> Self {
        Self {
            road_node_entity,
            direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoadSectionVariant {
    Straight,
    Curved(CircularArc),
}

#[derive(Clone, Copy, Debug)]
pub struct RequestedRoadSection {
    pub ends: [RequestedRoadSectionEnd; 2],
    pub variant: RoadSectionVariant,
}

impl RequestedRoadSection {
    /// Returns the RequestedRoadSectionEnd that is considered the "start" of this section.
    pub fn start(&self) -> RequestedRoadSectionEnd {
        self.ends[0]
    }

    /// Returns the RequestedRoadSectionEnd that is considered the "end" of this section.
    pub fn end(&self) -> RequestedRoadSectionEnd {
        self.ends[1]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RequestedRoadSectionEnd {
    /// The node this end is connected to.
    pub road_node: RequestedRoadNode,
    /// The outwards facing direction that this end looks towards.
    pub direction: Dir3,
}

impl RequestedRoadSectionEnd {
    // TODO: use these methods to build cross-section slices in RoadBuilder

    #[expect(unused)]
    pub fn outwards_transform(&self) -> Transform {
        self.transform_with_direction(self.direction)
    }

    #[expect(unused)]
    pub fn inwards_transform(&self) -> Transform {
        self.transform_with_direction(-self.direction)
    }

    pub fn transform_with_direction(&self, direction: Dir3) -> Transform {
        Transform::from_translation(self.road_node.position).looking_to(direction, Dir3::Y)
    }
}

fn calculate_road_section_transform(
    start_node_position: Vec3,
    end_node_position: Vec3,
) -> Transform {
    let delta_position = end_node_position - start_node_position;
    let section_position = start_node_position + delta_position / 2.0;

    Transform::from_translation(section_position).looking_at(end_node_position, Dir3::Y)
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
