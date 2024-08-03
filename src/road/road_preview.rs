use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::GameRunningSet;

use super::{road_builder::RoadBuilder, ActiveRoad, OnActiveRoadModified, RoadData};

pub struct RoadPreviewPlugin;

impl Plugin for RoadPreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            redraw_preview_on_modified.in_set(GameRunningSet::UpdateEntities),
        );
    }
}

#[derive(Component)]
pub struct RoadPreview;

fn redraw_preview_on_modified(
    mut on_modified: EventReader<OnActiveRoadModified>,
    mut commands: Commands,
    mut road_preview_query: Query<&mut Handle<Mesh>, With<RoadPreview>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for event in on_modified.read() {
        let road_mesh = meshes.add(build_road_mesh(event.road_data().clone()).get_mesh());

        if let Some(mut active_road_mesh) =
            get_road_preview(&mut road_preview_query, event.road_preview_entity())
        {
            *active_road_mesh = road_mesh;
        } else {
            let road_material = create_road_preview_material(&mut materials);
            let road_preview_entity = spawn_road_preview(&mut commands, road_mesh, road_material);
            active_road.set_road_preview_entity(Some(road_preview_entity));
        }
    }
}

fn get_road_preview<'a>(
    query: &'a mut Query<&mut Handle<Mesh>, With<RoadPreview>>,
    road_preview_entity: Option<Entity>,
) -> Option<Mut<'a, Handle<Mesh>>> {
    query.get_mut(road_preview_entity?).ok()
}

fn spawn_road_preview(
    commands: &mut Commands,
    road_mesh: Handle<Mesh>,
    road_material: Handle<StandardMaterial>,
) -> Entity {
    commands
        .spawn((
            RoadPreview,
            PbrBundle {
                mesh: road_mesh,
                material: road_material,
                ..default()
            },
        ))
        .id()
}

fn create_road_preview_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: GRAY_600.into(),
        perceptual_roughness: 0.7,
        ..default()
    })
}

fn build_road_mesh(road_data: RoadData) -> RoadBuilder {
    let mut road_builder = RoadBuilder::new();

    road_builder.build_from_road_data(road_data);

    road_builder
}
