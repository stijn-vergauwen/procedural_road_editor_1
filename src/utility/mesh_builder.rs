pub mod debug;
pub mod triangle_indices;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use bevy_rapier3d::prelude::*;

use self::{
    debug::{MeshBuilderDebugPlugin, MeshDebugData},
    triangle_indices::TriangleIndices,
};

pub struct MeshBuilderPlugin;

impl Plugin for MeshBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshBuilderDebugPlugin);
    }
}

const QUAD_CORNERS: [Vec3; 4] = [
    Vec3::new(-1.0, -1.0, 0.0),
    Vec3::new(-1.0, 1.0, 0.0),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(1.0, -1.0, 0.0),
];

#[derive(Debug)]
pub struct MeshBuilder {
    vertices: Vec<Vec3>,
    triangles: Vec<TriangleIndices>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
    vertex_count: u32,
    triangle_count: u32,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            vertex_count: 0,
            triangle_count: 0,
        }
    }

    /// Adds a single vertex with it's corresponding normal and uv to the mesh
    pub fn add_vertex(&mut self, vertex: Vec3, normal: Vec3, uv: Vec2) {
        self.vertices.push(vertex);
        self.normals.push(normal);
        self.uvs.push(uv);
        self.vertex_count += 1;
    }

    /// Adds a single triangle consisting of 3 indices.
    ///
    /// The indices should go in counter-clockwise order when looked at from the visible side.
    pub fn add_triangle(&mut self, triangle: TriangleIndices) {
        self.triangles.push(triangle);
        self.triangle_count += 1;
    }

    /// Builds a quad and adds it's vertices, normals, uvs, and triangles to the mesh
    ///
    /// Note: The uv parameter is used on every corner of the quad, color gradients are not possible with this function
    pub fn add_quad(&mut self, transform: Transform, uv: Vec2) {
        for corner_position in get_scaled_quad_corners(transform.scale).into_iter() {
            self.add_vertex(
                transform.translation + transform.rotation * corner_position,
                transform.forward().as_vec3(),
                uv,
            );
        }

        let vertex_i = self.vertex_index();

        self.add_triangle(TriangleIndices(vertex_i - 3, vertex_i - 2, vertex_i - 1));
        self.add_triangle(TriangleIndices(vertex_i - 3, vertex_i - 1, vertex_i));
    }

    /// Returns the index of the last vertex
    ///
    /// Used to offset triangle indices
    pub fn vertex_index(&self) -> u32 {
        self.vertex_count - 1
    }

    /// Returns if this MeshBuilder contains any vertices or triangles
    pub fn empty(&self) -> bool {
        self.vertex_count == 0 && self.triangle_count == 0
    }

    /// Generates a `MeshDebugData` object from the current mesh data
    ///
    /// * Uses `origin` to determine where in world space the mesh should be rendered
    #[expect(unused)]
    pub fn to_debug_data(
        &self,
        origin: Vec3,
        debug_vertices: bool,
        debug_triangles: bool,
    ) -> MeshDebugData {
        let vertices = debug_vertices.then_some(self.vertices.clone());
        let triangles = debug_triangles.then_some(self.triangles.clone());

        MeshDebugData::new(origin, vertices, triangles)
    }

    /// Generates a rapier `Collider` component from the current mesh data
    #[expect(unused)]
    pub fn to_collider(&self) -> Collider {
        Collider::trimesh(
            self.vertices.clone(),
            self.triangles
                .iter()
                .map(|triangle| triangle.to_array())
                .collect(),
        )
    }

    /// Generates a `Mesh` from the current data
    pub fn to_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vector3s_to_arrays(&self.vertices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vector3s_to_arrays(&self.normals));
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vector2s_to_arrays(&self.uvs));

        mesh.insert_indices(triangles_to_indices(&self.triangles));

        mesh
    }
}

fn vector3s_to_arrays(vectors: &Vec<Vec3>) -> Vec<[f32; 3]> {
    vectors.iter().map(|vertex| vertex.to_array()).collect()
}

fn vector2s_to_arrays(vectors: &Vec<Vec2>) -> Vec<[f32; 2]> {
    vectors.iter().map(|vertex| vertex.to_array()).collect()
}

fn triangles_to_indices(triangles: &Vec<TriangleIndices>) -> Indices {
    let as_indices = triangles
        .iter()
        .flat_map(|triangle| triangle.to_array())
        .collect();
    Indices::U32(as_indices)
}

fn get_scaled_quad_corners(scale: Vec3) -> [Vec3; 4] {
    QUAD_CORNERS.map(|corner| {
        Vec3::new(
            corner.x * scale.x / 2.0,
            corner.y * scale.y / 2.0,
            -scale.z / 2.0,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_vertex() {
        let mut builder = MeshBuilder::new();
        let vertex = Vec3::new(1.0, 1.0, 0.0);
        let normal = Vec3::Y;
        let uv = Vec2::ZERO;

        builder.add_vertex(vertex, normal, uv);

        assert_eq!(builder.vertex_count, 1);
        assert_eq!(builder.vertices[0], vertex);
        assert_eq!(builder.normals[0], normal);
        assert_eq!(builder.uvs[0], uv);
    }

    #[test]
    fn can_add_triangles() {
        let mut builder = MeshBuilder::new();
        let triangle_a = TriangleIndices(5, 6, 7);
        let triangle_b = TriangleIndices(3, 0, 1);

        builder.add_triangle(triangle_a);
        assert_eq!(builder.triangle_count, 1);

        builder.add_triangle(triangle_b);
        assert_eq!(builder.triangle_count, 2);

        assert_eq!(builder.triangles[0], triangle_a);
        assert_eq!(builder.triangles[1], triangle_b);
    }

    #[test]
    fn can_add_quad() {
        let mut builder = MeshBuilder::new();
        let transform = Transform::from_translation(Vec3::new(2.0, 2.0, 0.0));
        let uv = Vec2::ZERO;

        builder.add_quad(transform, uv);

        assert_eq!(builder.vertex_count, 4);
        assert_eq!(builder.triangle_count, 2);

        assert_eq!(builder.vertices[0], Vec3::new(1.5, 1.5, -0.5));
        assert_eq!(builder.vertices[2], Vec3::new(2.5, 2.5, -0.5));

        for normal in builder.normals.iter() {
            assert_eq!(*normal, Vec3::NEG_Z);
        }
    }

    #[test]
    fn can_check_if_empty() {
        let transform = Transform::from_translation(Vec3::new(2.0, 2.0, 0.0));
        let uv = Vec2::ZERO;

        let empty_builder = MeshBuilder::new();

        let mut builder_with_quad = MeshBuilder::new();
        builder_with_quad.add_quad(transform, uv);

        assert!(empty_builder.empty());
        assert!(!builder_with_quad.empty());
    }
}
