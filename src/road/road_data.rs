use std::{iter::Enumerate, slice::Iter};

use bevy::{math::NormedVectorSpace, prelude::*};
use serde::{Deserialize, Serialize};

use super::{road_component::RoadComponent, road_marking::RoadMarking};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadData {
    name: String,
    components: Vec<RoadComponent>,
    markings: Vec<RoadMarking>,
}

impl RoadData {
    pub fn new(name: String, components: Vec<RoadComponent>, markings: Vec<RoadMarking>) -> Self {
        Self {
            name,
            components,
            markings,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn components(&self) -> &[RoadComponent] {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut Vec<RoadComponent> {
        &mut self.components
    }

    pub fn enumerate_components(&self) -> Enumerate<Iter<RoadComponent>> {
        self.components.iter().enumerate()
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    pub fn markings(&self) -> &[RoadMarking] {
        &self.markings
    }

    pub fn markings_mut(&mut self) -> &mut Vec<RoadMarking> {
        &mut self.markings
    }

    pub fn enumerate_markings(&self) -> Enumerate<Iter<RoadMarking>> {
        self.markings.iter().enumerate()
    }

    #[allow(unused)]
    fn total_size(&self) -> Vec2 {
        Vec2::new(self.total_width(), self.total_height())
    }

    pub fn total_width(&self) -> f32 {
        self.components
            .iter()
            .fold(0.0, |sum, component| sum + component.size.x)
    }

    pub fn half_width(&self) -> f32 {
        self.total_width() / 2.0
    }

    #[allow(unused)]
    fn total_height(&self) -> f32 {
        self.components
            .iter()
            .fold(0.0, |sum, component| sum.max(component.size.y))
    }

    pub fn find_road_component_at_x_position(
        &self,
        x_position: f32,
    ) -> Option<(usize, RoadComponent)> {
        let road_component_positions = self.calculate_road_component_positions();

        self.components
            .clone()
            .into_iter()
            .enumerate()
            .find(|(index, component)| {
                let component_position = road_component_positions[*index];

                x_position.distance(component_position) <= component.half_width()
            })
    }

    /// Returns an array describing the x_position of each component relative to the road's center.
    pub fn calculate_road_component_positions(&self) -> Vec<f32> {
        let mut width_of_previous_components = 0.0;
        let mut result = Vec::new();

        for component in self.components() {
            let x_position =
                width_of_previous_components + component.half_width() - self.half_width();
            result.push(x_position);

            width_of_previous_components += component.width();
        }

        result
    }
}

impl Default for RoadData {
    fn default() -> Self {
        Self {
            name: String::from("New road"),
            components: Vec::new(),
            markings: Vec::new(),
        }
    }
}
