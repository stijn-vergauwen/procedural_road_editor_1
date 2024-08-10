use bevy::prelude::*;

use crate::{utility::texture_builder::TextureBuilder, GameRunningSet};

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

// TODO: split system in 2, one for create one for update
fn redraw_preview_on_modified(
    mut on_modified: EventReader<OnActiveRoadModified>,
    mut commands: Commands,
    mut road_preview_query: Query<
        (&mut Handle<Mesh>, &Handle<StandardMaterial>),
        With<RoadPreview>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for event in on_modified.read() {
        let road_mesh = meshes.add(build_road_mesh(event.road_data().clone()).get_mesh());
        let road_texture_image = images.add(road_texture_from_road_data(event.road_data()));

        if let Some((mut active_road_mesh, active_road_material)) = event
            .road_preview_entity()
            .map(|road_preview_entity| road_preview_query.get_mut(road_preview_entity).ok())
            .flatten()
        {
            *active_road_mesh = road_mesh;

            let road_material = materials.get_mut(active_road_material).unwrap();
            road_material.base_color_texture = Some(road_texture_image);
        } else {
            let road_material = create_road_preview_material(&mut materials, road_texture_image);
            let road_preview_entity = spawn_road_preview(&mut commands, road_mesh, road_material);

            active_road.set_road_preview_entity(Some(road_preview_entity));
        }
    }
}

fn road_texture_from_road_data(road_data: &RoadData) -> Image {
    TextureBuilder::image_from_colors(
        road_data
            .components()
            .iter()
            .map(|component| component.color())
            .collect(),
    )
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
    materials: &mut Assets<StandardMaterial>,
    texture_image: Handle<Image>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color_texture: Some(texture_image),
        perceptual_roughness: 0.7,
        ..default()
    })
}

fn build_road_mesh(road_data: RoadData) -> RoadBuilder {
    let mut road_builder = RoadBuilder::new();

    road_builder.build_from_road_data(road_data);

    road_builder
}
