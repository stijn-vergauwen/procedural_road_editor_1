use bevy::prelude::*;


/// RoadNodes are the points in a road network, they're connected by RoadSections.
/// - Nodes describe start & end position while Sections describe how they're connected.
#[derive(Component, Debug)]
pub struct RoadNode {
    connected_sections: Vec<Entity>
}

impl RoadNode {
    pub fn new(connected_sections: Vec<Entity>) -> Self {
        Self { connected_sections }
    }
}