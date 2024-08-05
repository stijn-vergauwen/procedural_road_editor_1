use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct RoadComponent {
    name: String,
    size: Vec2,
}

impl RoadComponent {
    pub fn new(name: impl Into<String>, size: Vec2) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }

    pub fn with_size(&mut self, size: Vec2) -> &mut Self {
        self.size = size;

        self
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;

        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }
}
