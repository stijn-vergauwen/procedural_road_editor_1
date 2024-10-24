use bevy::prelude::*;

use crate::utility::{mesh_builder::MeshBuilder, texture_builder::TextureBuilder};

use super::{road_data::RoadData, road_marking::RoadMarking};

/// Builds the 3D road mesh from the given road data.
pub struct RoadBuilder {
    mesh_builder: MeshBuilder,
    texture_builder: TextureBuilder,
    road_length: f32,
}

impl RoadBuilder {
    pub fn new(road_length: f32) -> Self {
        Self {
            mesh_builder: MeshBuilder::new(),
            texture_builder: TextureBuilder::new(),
            road_length,
        }
    }

    pub fn build_from_road_data(&mut self, road_data: &RoadData) {
        if !self.mesh_builder.empty() {
            warn!("build method called on a RoadBuilder that already contains mesh data.");
        }

        let road_component_length = road_data.components().len();
        let road_texture_length = road_component_length + road_data.markings().len();
        
        self.build_road_texture(road_data);
        self.build_road_components(road_data, road_texture_length);
        self.build_road_markings(road_data, road_texture_length, road_component_length);
    }

    pub fn get_mesh(&self) -> Mesh {
        self.mesh_builder.to_mesh()
    }

    pub fn get_texture_image(&self) -> Image {
        self.texture_builder.build_texture_image()
    }

    fn build_road_texture(&mut self, road_data: &RoadData) {
        let road_component_colors: Vec<Color> = road_data
            .components()
            .iter()
            .map(|component| component.color)
            .collect();

        let road_marking_colors: Vec<Color> = road_data
            .markings()
            .iter()
            .map(|marking| marking.color)
            .collect();

        self.texture_builder
            .add_colors([road_component_colors, road_marking_colors].concat());
    }

    fn build_road_components(&mut self, road_data: &RoadData, road_texture_length: usize) {
        let mut width_of_built_sections = 0.0;

        for (index, component) in road_data.enumerate_components() {
            self.build_road_component(
                &mut width_of_built_sections,
                component.size,
                road_data.total_width(),
                calculate_road_component_uv(index, road_texture_length),
            )
        }
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
            calculate_left_face_transform(x_position, component_size, self.road_length),
            uv,
        );

        // Build top quad
        self.mesh_builder.add_quad(
            calculate_top_face_transform(x_position, component_size, self.road_length),
            uv,
        );

        // Build right side quad
        self.mesh_builder.add_quad(
            calculate_right_face_transform(x_position, component_size, self.road_length),
            uv,
        );

        *width_of_built_sections += component_size.x;
    }

    fn build_road_markings(
        &mut self,
        road_data: &RoadData,
        road_texture_length: usize,
        road_component_length: usize,
    ) {
        for (index, road_marking) in road_data.enumerate_markings() {
            self.build_road_marking(
                *road_marking,
                calculate_road_marking_uv(index, road_texture_length, road_component_length),
                road_data,
            );
        }
    }

    fn build_road_marking(&mut self, road_marking: RoadMarking, uv: Vec2, road_data: &RoadData) {
        let road_height = road_data
            .find_road_component_under_point(road_marking.x_position)
            .unwrap()
            .road_component
            .size
            .y;

        let size = Vec2::new(road_marking.segment_width, road_height + 0.01);
        self.mesh_builder.add_quad(
            calculate_top_face_transform(road_marking.x_position, size, self.road_length),
            uv,
        );
    }
}

// Utils

fn calculate_left_face_transform(x_position: f32, size: Vec2, road_length: f32) -> Transform {
    Transform::from_translation(Vec3::X * x_position)
        .with_scale(Vec3::new(road_length, size.y, size.x))
        .looking_to(Vec3::NEG_X, Vec3::Y)
}

fn calculate_top_face_transform(x_position: f32, size: Vec2, road_length: f32) -> Transform {
    Transform::from_translation(Vec3::X * x_position)
        .with_scale(Vec3::new(road_length, size.x, size.y))
        .looking_to(Vec3::Y, Vec3::X)
}

fn calculate_right_face_transform(x_position: f32, size: Vec2, road_length: f32) -> Transform {
    Transform::from_translation(Vec3::X * x_position)
        .with_scale(Vec3::new(road_length, size.y, size.x))
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
