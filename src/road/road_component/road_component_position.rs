use serde::{Deserialize, Serialize};

/// Holds x positions of a components left side, center, and right side.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RoadComponentPosition {
    pub left: f32,
    pub center: f32,
    pub right: f32,
}

impl RoadComponentPosition {
    pub fn new(left: f32, center: f32, right: f32) -> Self {
        Self {
            left,
            center,
            right,
        }
    }
}
