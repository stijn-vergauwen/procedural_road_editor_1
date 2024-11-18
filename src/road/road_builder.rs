use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::utility::{
    mesh_builder::{debug::MeshDebugData, triangle_indices::TriangleIndices, MeshBuilder},
    texture_builder::TextureBuilder,
};

use super::{
    road_data::RoadData,
    road_section::{RequestedRoadSection, RoadSectionVariant},
};

/// Builds the 3D road mesh from the given road data.
pub struct RoadBuilder {
    mesh_builder: MeshBuilder,
    texture_builder: TextureBuilder,
}

impl RoadBuilder {
    pub fn new() -> Self {
        Self {
            mesh_builder: MeshBuilder::new(),
            texture_builder: TextureBuilder::new(),
        }
    }

    pub fn build_from_requested_section(
        &mut self,
        requested_section: &RequestedRoadSection,
        road_design: &RoadData,
    ) {
        if !self.mesh_builder.empty() {
            warn!("build method called on a RoadBuilder that already contains mesh data.");
        }

        let road_component_length = road_design.components().len();
        let road_texture_length = road_component_length + road_design.markings().len();

        self.build_road_texture(road_design);
        self.build_mesh_from_slice_transforms(
            road_design,
            get_slice_transforms_from_requested_section(requested_section),
            road_texture_length,
        );
        // self.build_road_markings(road_design, road_texture_length, road_component_length);
    }

    pub fn get_mesh(&self) -> Mesh {
        self.mesh_builder.to_mesh()
    }

    pub fn get_texture_image(&self) -> Image {
        self.texture_builder.build_texture_image()
    }

    pub fn get_collider(&self) -> Collider {
        self.mesh_builder.to_collider()
    }

    #[allow(unused)]
    pub fn to_debug_data(
        &self,
        origin: Vec3,
        debug_normals: bool,
        debug_triangles: bool,
    ) -> MeshDebugData {
        self.mesh_builder
            .to_debug_data(origin, debug_normals, debug_triangles)
    }

    fn build_road_texture(&mut self, road_design: &RoadData) {
        let road_component_colors: Vec<Color> = road_design
            .components()
            .iter()
            .map(|component| component.color)
            .collect();

        let road_marking_colors: Vec<Color> = road_design
            .markings()
            .iter()
            .map(|marking| marking.color)
            .collect();

        self.texture_builder
            .add_colors([road_component_colors, road_marking_colors].concat());
    }

    // TODO: rework road markings to work with the new cross-section method
    // fn build_road_markings(
    //     &mut self,
    //     road_design: &RoadData,
    //     road_texture_length: usize,
    //     road_component_length: usize,
    // ) {
    //     for (index, road_marking) in road_design.enumerate_markings() {
    //         self.build_road_marking(
    //             *road_marking,
    //             calculate_road_marking_uv(index, road_texture_length, road_component_length),
    //             road_design,
    //         );
    //     }
    // }

    // fn build_road_marking(&mut self, road_marking: RoadMarking, uv: Vec2, road_data: &RoadData) {
    //     let road_height = road_data
    //         .find_road_component_under_point(road_marking.x_position)
    //         .unwrap()
    //         .road_component
    //         .size
    //         .y;

    //     let size = Vec2::new(road_marking.segment_width, road_height + 0.01);
    //     self.mesh_builder.add_quad(
    //         calculate_top_face_transform(road_marking.x_position, size, self.road_length),
    //         uv,
    //     );
    // }

    fn build_mesh_from_slice_transforms(
        &mut self,
        road_design: &RoadData,
        slice_transforms: Vec<Transform>,
        road_texture_length: usize,
    ) {
        let road_design_slice = calculate_road_design_slice(road_design, road_texture_length);

        for (slice_index, slice_transform) in slice_transforms.iter().enumerate() {
            let translation = slice_transform.translation;
            let rotation = slice_transform.rotation;

            let transformed_vertices: Vec<Vec3> = road_design_slice
                .vertices
                .iter()
                .map(|vertex| translation + rotation * *vertex)
                .collect();

            let transformed_normals: Vec<Vec3> = road_design_slice
                .normals
                .iter()
                .map(|normal| rotation * *normal)
                .collect();

            for vertex_index in 0..road_design_slice.len() {
                self.mesh_builder.add_vertex(
                    transformed_vertices[vertex_index],
                    transformed_normals[vertex_index],
                    road_design_slice.uvs[vertex_index],
                );
            }

            if slice_index < slice_transforms.len() - 1 {
                for triangle in calculate_road_design_slice_triangles(
                    slice_index as u32,
                    road_design_slice.len() as u32,
                ) {
                    self.mesh_builder.add_triangle(triangle);
                }
            }
        }
    }
}

// New Utils

/// A cross-section slice for the given road_design, made out of vertices with their corresponding normals and UVs.
///
/// - Vertex positions and normals are relative to origin and without rotation.
/// - The order of the vertices is clockwise, starting on the bottom-left.
pub struct RoadDesignSlice {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
}

impl RoadDesignSlice {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(capacity),
            normals: Vec::with_capacity(capacity),
            uvs: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, vertex: Vec3, normal: Vec3, uv: Vec2) {
        self.vertices.push(vertex);
        self.normals.push(normal);
        self.uvs.push(uv);
    }

    fn len(&self) -> usize {
        assert_eq!(self.vertices.len(), self.normals.len());
        assert_eq!(self.vertices.len(), self.uvs.len());

        self.vertices.len()
    }
}

struct SideToDraw {
    lower_height: f32,
    upper_height: f32,
}

impl SideToDraw {
    fn new(lower_height: f32, upper_height: f32) -> Self {
        Self {
            lower_height,
            upper_height,
        }
    }
}

/// Returns a cross-section slice made out of vertices for the given road_design.
///
/// - Vertex positions are relative to origin and without rotation.
/// - The order of the vertices is clockwise, starting on the bottom-left.
pub fn calculate_road_design_slice(
    road_design: &RoadData,
    road_texture_length: usize,
) -> RoadDesignSlice {
    let min_vertices_count = road_design.component_count() + 3;
    let mut road_design_slice = RoadDesignSlice::with_capacity(min_vertices_count);

    let mut width_of_built_components = 0.0;

    for (index, current_component) in road_design.enumerate_components() {
        let uv = calculate_road_component_uv(index, road_texture_length);

        let left_x_position = calculate_left_x_position_of_road_component(
            road_design.total_width(),
            width_of_built_components,
        );
        let right_x_position = left_x_position + current_component.size.x;

        let current_height = current_component.height();

        let previous_height = if index > 0 {
            road_design.components()[index - 1].height()
        } else {
            0.0
        };

        let left_side = (previous_height < current_height)
            .then_some(SideToDraw::new(previous_height, current_height));

        let next_height = if index < road_design.component_count() - 1 {
            road_design.components()[index + 1].height()
        } else {
            0.0
        };

        let right_side =
            (next_height < current_height).then_some(SideToDraw::new(next_height, current_height));

        // Left side
        if let Some(side_to_draw) = left_side {
            road_design_slice.push(
                Vec3::new(left_x_position, side_to_draw.lower_height, 0.0),
                Vec3::NEG_X,
                uv,
            );

            road_design_slice.push(
                Vec3::new(left_x_position, side_to_draw.upper_height, 0.0),
                Vec3::NEG_X,
                uv,
            );
        }

        // Top side
        road_design_slice.push(
            Vec3::new(left_x_position, current_height, 0.0),
            Vec3::Y,
            uv,
        );

        road_design_slice.push(
            Vec3::new(right_x_position, current_height, 0.0),
            Vec3::Y,
            uv,
        );

        // Right side
        if let Some(side_to_draw) = right_side {
            road_design_slice.push(
                Vec3::new(right_x_position, side_to_draw.upper_height, 0.0),
                Vec3::X,
                uv,
            );

            road_design_slice.push(
                Vec3::new(right_x_position, side_to_draw.lower_height, 0.0),
                Vec3::X,
                uv,
            );
        }

        width_of_built_components += current_component.size.x;
    }

    road_design_slice
}

/// Returns a list of TriangleIndices that connect the current slice to the next slice.
///
/// - `slice_index` is the n-th slice that this fn is called for.
fn calculate_road_design_slice_triangles(
    slice_index: u32,
    vertices_in_slice: u32,
) -> Vec<TriangleIndices> {
    let vertices_in_mesh = slice_index * vertices_in_slice;
    let mut triangle_indices = Vec::new();

    for index in 0..vertices_in_slice - 1 {
        if !u32_is_even(index) {
            continue;
        }

        triangle_indices.push(
            TriangleIndices(
                index,
                index + vertices_in_slice + 1,
                index + vertices_in_slice,
            )
            .offsetted(vertices_in_mesh),
        );

        triangle_indices.push(
            TriangleIndices(index, index + 1, index + vertices_in_slice + 1)
                .offsetted(vertices_in_mesh),
        );
    }

    triangle_indices
}

fn get_slice_transforms_from_requested_section(
    requested_section: &RequestedRoadSection,
) -> Vec<Transform> {
    match requested_section.variant {
        RoadSectionVariant::Straight => {
            vec![
                requested_section.start().inwards_transform(),
                requested_section.end().outwards_transform(),
            ]
        }
        RoadSectionVariant::Curved(circular_arc) => {
            // let section_transform = calculate_road_section_transform(
            //     requested_section.start().road_node.position,
            //     requested_section.end().road_node.position,
            // );
            let transform_count = ((circular_arc.length().abs() * 0.5).round() as u32).max(5);
            let transform_direction = circular_arc.forwards_direction();
            circular_arc
                .calculate_transforms_along_arc(transform_count, transform_direction)
                .iter()
                .map(|arc_transform| {
                    // TODO: transform each "arc_transform" from "relative to circular arc" to "relative to road section"
                    // arc_transform
                    //     .with_translation(
                    //         section_transform.rotation.inverse() * arc_transform.translation
                    //             - (section_transform.translation - circular_arc.position),
                    //     )
                    //     .with_rotation(
                    //         arc_transform.rotation + section_transform.rotation.inverse(),
                    //     )

                    arc_transform
                        .with_translation(arc_transform.translation + circular_arc.position)
                })
                .collect()
        }
    }
}

/// Returns the x position of a component without taking it's size into account, so on the far-left side of the component.
fn calculate_left_x_position_of_road_component(
    road_width: f32,
    width_of_built_components: f32,
) -> f32 {
    -road_width / 2.0 + width_of_built_components
}

#[expect(unused)]
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

fn u32_is_even(num: u32) -> bool {
    (1 - ((num & 1) << 1) as i32) == 1
}
