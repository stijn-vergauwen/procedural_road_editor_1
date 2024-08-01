use bevy::{color::palettes::tailwind::GRAY_600, prelude::*};

use crate::{utility::mesh_builder::MeshBuilder, GameRunningSet};

use super::{ActiveRoad, OnActiveRoadModified, RoadData, RoadEditor};

const ROAD_LENGTH: f32 = 20.0;

pub struct RoadBuilderPlugin;

impl Plugin for RoadBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            redraw_road_on_modified.in_set(GameRunningSet::UpdateEntities),
        );
    }
}

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

        // TODO: refactor

        let road_width = road_data.total_width();
        let mut width_of_built_sections = 0.0;

        for component in road_data.components() {
            let size = component.size();
            let x_position = -road_width / 2.0 + width_of_built_sections + size.x / 2.0;
            width_of_built_sections += size.x;

            // Build left side quad
            let transform = Transform::from_translation(Vec3::X * x_position)
                .with_scale(Vec3::new(ROAD_LENGTH, size.y, size.x))
                .looking_to(Vec3::NEG_X, Vec3::Y);

            self.mesh_builder.add_quad(transform, Vec2::new(0.0, 0.0));

            // Build top quad
            let transform = Transform::from_translation(Vec3::X * x_position)
                .with_scale(Vec3::new(ROAD_LENGTH, size.x, size.y))
                .looking_to(Vec3::Y, Vec3::X);

            self.mesh_builder.add_quad(transform, Vec2::new(0.0, 0.0));

            // Build right side quad
            let transform = Transform::from_translation(Vec3::X * x_position)
                .with_scale(Vec3::new(ROAD_LENGTH, size.y, size.x))
                .looking_to(Vec3::X, Vec3::Y);

            self.mesh_builder.add_quad(transform, Vec2::new(0.0, 0.0));
        }
    }

    pub fn get_mesh(&self) -> Mesh {
        self.mesh_builder.to_mesh()
    }
}

fn redraw_road_on_modified(
    mut on_modified: EventReader<OnActiveRoadModified>,
    mut commands: Commands,
    mut active_road_query: Query<&mut Handle<Mesh>, With<ActiveRoad>>,
    road_editor: Res<RoadEditor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in on_modified.read() {
        println!("Redraw road!");

        // TODO: refactor

        let mut road_builder = RoadBuilder::new();
        road_builder.build_from_road_data(road_editor.road().clone());

        let road_mesh = meshes.add(road_builder.get_mesh());

        if let Ok(mut active_road_mesh) = active_road_query.get_single_mut() {
            *active_road_mesh = road_mesh;
        } else {
            commands.spawn((
                ActiveRoad,
                PbrBundle {
                    mesh: road_mesh,
                    material: materials.add(StandardMaterial {
                        base_color: GRAY_600.into(),
                        perceptual_roughness: 0.7,
                        ..default()
                    }),
                    ..default()
                },
            ));
        }
    }
}
