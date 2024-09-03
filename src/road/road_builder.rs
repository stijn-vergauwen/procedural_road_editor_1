use bevy::prelude::*;

use crate::utility::mesh_builder::MeshBuilder;

use super::{road_marking::RoadMarking, road_data::RoadData};

const ROAD_LENGTH: f32 = 20.0;

pub struct RoadBuilder {
    mesh_builder: MeshBuilder,
}

impl RoadBuilder {
    pub fn new() -> Self {
        Self {
            mesh_builder: MeshBuilder::new(),
        }
    }

    pub fn build_from_road_data(&mut self, road_data: RoadData) {
        if !self.mesh_builder.empty() {
            warn!("build method called on a RoadBuilder that already contains mesh data.");
        }

        let road_component_length = road_data.components().len();
        let road_texture_length = road_component_length + road_data.markings().len();
        let mut width_of_built_sections = 0.0;

        for (index, component) in road_data.enumerate_components() {
            self.build_road_component(
                &mut width_of_built_sections,
                component.size,
                road_data.total_width(),
                calculate_road_component_uv(index, road_texture_length),
            )
        }

        for (index, road_marking) in road_data.enumerate_markings() {
            self.build_road_marking(
                *road_marking,
                calculate_road_marking_uv(index, road_texture_length, road_component_length),
                &road_data,
            );
        }
    }

    pub fn get_mesh(&self) -> Mesh {
        self.mesh_builder.to_mesh()
    }

    fn build_road_component(
        &mut self,
        width_of_built_sections: &mut f32,
        component_size: Vec2,
        road_width: f32,
        uv: Vec2,
    ) {
        let x_position = calculate_x_position_of_road_component(
            road_width,
            *width_of_built_sections,
            component_size,
        );

        // Build left side quad
        self.mesh_builder.add_quad(
            calculate_left_face_transform(x_position, component_size),
            uv,
        );

        // Build top quad
        self.mesh_builder
            .add_quad(calculate_top_face_transform(x_position, component_size), uv);

        // Build right side quad
        self.mesh_builder.add_quad(
            calculate_right_face_transform(x_position, component_size),
            uv,
        );

        *width_of_built_sections += component_size.x;
    }

    fn build_road_marking(&mut self, road_marking: RoadMarking, uv: Vec2, road_data: &RoadData) {
        let road_height = road_data
            .find_road_component_at_x_position(road_marking.x_position)
            .unwrap()
            .size
            .y;

        let size = Vec2::new(road_marking.segment_width, road_height + 0.01);
        self.mesh_builder.add_quad(
            calculate_top_face_transform(road_marking.x_position, size),
            uv,
        );
    }
}

// Utils

fn calculate_left_face_transform(x_position: f32, size: Vec2) -> Transform {
    Transform::from_translation(Vec3::X * x_position)
        .with_scale(Vec3::new(ROAD_LENGTH, size.y, size.x))
        .looking_to(Vec3::NEG_X, Vec3::Y)
}

fn calculate_top_face_transform(x_position: f32, size: Vec2) -> Transform {
    Transform::from_translation(Vec3::X * x_position)
        .with_scale(Vec3::new(ROAD_LENGTH, size.x, size.y))
        .looking_to(Vec3::Y, Vec3::X)
}

fn calculate_right_face_transform(x_position: f32, size: Vec2) -> Transform {
    Transform::from_translation(Vec3::X * x_position)
        .with_scale(Vec3::new(ROAD_LENGTH, size.y, size.x))
        .looking_to(Vec3::X, Vec3::Y)
}

fn calculate_x_position_of_road_component(
    road_width: f32,
    width_of_built_sections: f32,
    size: Vec2,
) -> f32 {
    -road_width / 2.0 + width_of_built_sections + size.x / 2.0
}

fn calculate_road_marking_uv(
    index: usize,
    texture_length: usize,
    road_component_length: usize,
) -> Vec2 {
    calculate_road_component_uv(index + road_component_length, texture_length)
}

fn calculate_road_component_uv(index: usize, texture_length: usize) -> Vec2 {
    let step_size = 1.0 / texture_length as f32;
    Vec2::new(step_size * index as f32 + step_size * 0.5, 0.5)
}
