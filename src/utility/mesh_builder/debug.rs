use bevy::{color::palettes::tailwind::*, prelude::*};

use super::TriangleIndices;

pub struct MeshBuilderDebugPlugin;

impl Plugin for MeshBuilderDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_debug_meshes);
    }
}

const VERTEX_COLOR: Srgba = LIME_400;
const VERTEX_RADIUS: f32 = 0.15;
const TRIANGLE_COLOR: Srgba = BLUE_500;
const NORMAL_COLOR: Srgba = AMBER_400;

#[derive(Component)]
pub struct MeshDebugData {
    origin: Vec3,
    vertices: Vec<Vec3>,
    normals: Option<Vec<Vec3>>,
    triangles: Option<Vec<TriangleIndices>>,
}

impl MeshDebugData {
    pub fn new(
        origin: Vec3,
        vertices: Vec<Vec3>,
        normals: Option<Vec<Vec3>>,
        triangles: Option<Vec<TriangleIndices>>,
    ) -> Self {
        Self {
            origin,
            vertices,
            normals,
            triangles,
        }
    }
}

fn draw_debug_meshes(mut gizmos: Gizmos, debug_meshes: Query<&MeshDebugData>) {
    for mesh in debug_meshes.iter() {
        draw_mesh_data(&mut gizmos, mesh);
    }
}

fn draw_mesh_data(gizmos: &mut Gizmos, data: &MeshDebugData) {
    for &vertex in data.vertices.iter() {
        draw_sphere(gizmos, data.origin + vertex);
    }

    if let Some(normals) = &data.normals {
        for (index, normal) in normals.iter().enumerate() {
            let position = data.vertices[index];
            draw_ray(gizmos, position, *normal);
        }
    }

    if let Some(triangles) = &data.triangles {
        for &triangle in triangles.iter() {
            draw_triangle(
                gizmos,
                data.origin + data.vertices[triangle.0 as usize],
                data.origin + data.vertices[triangle.1 as usize],
                data.origin + data.vertices[triangle.2 as usize],
            );
        }
    }
}

fn draw_triangle(gizmos: &mut Gizmos, a: Vec3, b: Vec3, c: Vec3) {
    gizmos.line(a, b, TRIANGLE_COLOR);
    gizmos.line(b, c, TRIANGLE_COLOR);
    gizmos.line(c, a, TRIANGLE_COLOR);
}

fn draw_sphere(gizmos: &mut Gizmos, position: Vec3) {
    gizmos.sphere(position, Quat::IDENTITY, VERTEX_RADIUS, VERTEX_COLOR);
}

fn draw_ray(gizmos: &mut Gizmos, position: Vec3, direction: Vec3) {
    gizmos.ray(position, direction, NORMAL_COLOR);
}
