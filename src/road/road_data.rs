use std::{iter::Enumerate, slice::Iter};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    road_component::{road_component_position::RoadComponentPosition, RoadComponent},
    road_marking::RoadMarking,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoadData {
    name: String,
    components: Vec<RoadComponent>,
    component_positions: Vec<RoadComponentPosition>,
    markings: Vec<RoadMarking>,
}

impl RoadData {
    pub fn new(name: String, components: Vec<RoadComponent>, markings: Vec<RoadMarking>) -> Self {
        let component_positions = calculate_road_component_positions(&components);

        Self {
            name,
            components,
            component_positions,
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

    pub fn component_positions(&self) -> &[RoadComponentPosition] {
        &self.component_positions
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
        calculate_width_of_road_components(&self.components)
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

    pub fn recalculate_road_component_positions(&mut self) {
        self.component_positions = calculate_road_component_positions(&self.components);
    }

    pub fn find_road_component_at_x_position(
        &self,
        x_position: f32,
    ) -> Option<(usize, &RoadComponent)> {
        self.components.iter().enumerate().find(|(index, _)| {
            let component_position = self.component_positions[*index];

            x_position >= component_position.left && x_position <= component_position.right
        })
    }
}

impl Default for RoadData {
    fn default() -> Self {
        Self {
            name: String::from("New road"),
            components: Vec::new(),
            component_positions: Vec::new(),
            markings: Vec::new(),
        }
    }
}

pub fn calculate_road_component_positions(
    road_components: &[RoadComponent],
) -> Vec<RoadComponentPosition> {
    let half_width = calculate_width_of_road_components(road_components) / 2.0;

    let mut width_of_previous_components = 0.0;
    let mut result = Vec::new();

    for component in road_components {
        let left_position = width_of_previous_components - half_width;
        let center_position = left_position + component.half_width();
        let right_position = left_position + component.width();

        result.push(RoadComponentPosition::new(
            left_position,
            center_position,
            right_position,
        ));

        width_of_previous_components += component.width();
    }

    result
}

fn calculate_width_of_road_components(road_components: &[RoadComponent]) -> f32 {
    road_components
        .iter()
        .fold(0.0, |sum, component| sum + component.size.x)
}
