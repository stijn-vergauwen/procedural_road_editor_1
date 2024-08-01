use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::RoadComponent;

#[derive(Clone, Serialize, Deserialize)]
pub struct RoadData {
    name: String,
    components: Vec<RoadComponent>,
}

impl RoadData {
    pub fn new(name: String, components: Vec<RoadComponent>) -> Self {
        Self { name, components }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn components(&self) -> &[RoadComponent] {
        &self.components
    }
    
    pub fn components_mut(&mut self) -> &mut Vec<RoadComponent> {
        &mut self.components
    }

    #[allow(unused)]
    fn total_size(&self) -> Vec2 {
        Vec2::new(self.total_width(), self.total_height())
    }

    pub fn total_width(&self) -> f32 {
        self.components
            .iter()
            .fold(0.0, |sum, component| sum + component.size().x)
    }

    #[allow(unused)]
    fn total_height(&self) -> f32 {
        self.components
            .iter()
            .fold(0.0, |sum, component| sum.max(component.size().y))
    }
}
