pub mod road_component_position;

use bevy::{color::palettes::tailwind::GRAY_600, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default)]
pub struct RoadComponent {
    pub name: String,
    pub size: Vec2,
    pub color: Color,
}

impl RoadComponent {
    pub fn new(name: impl Into<String>, size: Vec2, color: impl Into<Color>) -> Self {
        Self {
            name: name.into(),
            size,
            color: color.into(),
        }
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.size = self.size.with_x(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.size = self.size.with_y(height);
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

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn half_width(&self) -> f32 {
        self.width() / 2.0
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

#[derive(Component, Clone, Copy, PartialEq)]
pub enum RoadComponentField {
    Name,
    Width,
    Height,
    Color,
}
