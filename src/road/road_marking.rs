pub mod config_ui;
pub mod road_marking_events;

use bevy::prelude::*;
use config_ui::RoadMarkingConfigUiPlugin;
use road_marking_events::RoadMarkingEventsPlugin;
use serde::{Deserialize, Serialize};

pub struct RoadMarkingPlugin;

impl Plugin for RoadMarkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RoadMarkingConfigUiPlugin, RoadMarkingEventsPlugin));
    }
}

// TODO: add segment_length field
// TODO: add segment_spacing field

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoadMarking {
    /// This marking's horizontal position relative to the road center.
    pub x_position: f32,
    pub segment_width: f32,
    pub color: Color,
}

impl RoadMarking {
    pub fn new(x_position: f32, segment_width: f32, color: impl Into<Color>) -> Self {
        Self {
            x_position,
            segment_width,
            color: color.into(),
        }
    }

    pub fn with_x_position(mut self, x_position: f32) -> Self {
        self.x_position = x_position;
        self
    }

    pub fn with_segment_width(mut self, segment_width: f32) -> Self {
        self.segment_width = segment_width;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum RoadMarkingField {
    XPosition,
    SegmentWidth,
    Color,
}
