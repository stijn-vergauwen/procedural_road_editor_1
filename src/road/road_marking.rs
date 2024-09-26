use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct RoadMarkingPlugin;

impl Plugin for RoadMarkingPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, ());
    }
}

// TODO: add segment_length field
// TODO: add segment_spacing field

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoadMarking {
    pub color: Color,
    pub segment_width: f32,
    /// This marking's horizontal position relative to the road center.
    pub x_position: f32,
}

impl RoadMarking {
    pub fn new(color: impl Into<Color>, segment_width: f32, x_position: f32) -> Self {
        Self {
            color: color.into(),
            segment_width,
            x_position,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_segment_width(mut self, segment_width: f32) -> Self {
        self.segment_width = segment_width;
        self
    }

    pub fn with_x_position(mut self, x_position: f32) -> Self {
        self.x_position = x_position;
        self
    }
}

