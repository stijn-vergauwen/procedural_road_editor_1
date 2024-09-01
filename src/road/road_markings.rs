use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct RoadMarkingsPlugin;

impl Plugin for RoadMarkingsPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, ());
    }
}

// TODO: add segment_length field
// TODO: add segment_spacing field

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct RoadMarking {
    pub color: Color,
    pub segment_width: f32,
    /// This marking's offset from the lane's center.
    pub offset: f32,
}

impl RoadMarking {
    pub fn new(color: Color, segment_width: f32, offset: f32) -> Self {
        Self {
            color,
            segment_width,
            offset,
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

    pub fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }
}
