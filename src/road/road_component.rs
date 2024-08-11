use bevy::{color::palettes::tailwind::GRAY_600, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadComponent {
    name: String,
    size: Vec2,
    color: Color,
}

impl RoadComponent {
    pub fn new(name: impl Into<String>, size: Vec2, color: Color) -> Self {
        Self {
            name: name.into(),
            size,
            color,
        }
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;

        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;

        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Default for RoadComponent {
    fn default() -> Self {
        Self {
            name: String::from("New component"),
            size: Vec2::new(1.0, 0.2),
            color: GRAY_600.into(),
        }
    }
}
