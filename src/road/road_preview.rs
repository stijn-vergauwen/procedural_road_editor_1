use bevy::prelude::*;

use crate::{utility::texture_builder::TextureBuilder, GameRunningSet};

use super::{
    active_road::{OnActiveRoadModified, OnActiveRoadSet},
    road_builder::RoadBuilder,
    ActiveRoad, RoadData,
};

pub struct RoadPreviewPlugin;

impl Plugin for RoadPreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                redraw_preview_on_active_road_modified,
                redraw_preview_on_active_road_set,
                spawn_preview_on_active_road_set,
            )
                .chain()
                .in_set(GameRunningSet::UpdateEntities),
        );
    }
}

#[derive(Component)]
pub struct RoadPreview;

/// Spawns in road preview in case it didn't already exists
fn spawn_preview_on_active_road_set(
    mut on_set: EventReader<OnActiveRoadSet>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut active_road: ResMut<ActiveRoad>,
    road_preview_query: Query<&RoadPreview>,
) {
    if !road_preview_query.is_empty() {
        return;
    }

    for event in on_set.read() {
        let (road_mesh, road_texture_image) =
            create_road_mesh_and_texture(&mut meshes, &mut images, event.road_data());

        let road_material = create_road_preview_material(&mut materials, road_texture_image);
        let road_preview_entity = commands
            .spawn(build_road_preview_bundle(road_mesh, road_material))
            .id();

        active_road.set_road_preview_entity(Some(road_preview_entity));
    }
}

/// Redraws road preview in case it already exists
fn redraw_preview_on_active_road_set(
    mut on_set: EventReader<OnActiveRoadSet>,
    mut road_preview_query: Query<
        (&mut Handle<Mesh>, &Handle<StandardMaterial>),
        With<RoadPreview>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in on_set.read() {
        let Ok((mut preview_mesh, preview_material)) = road_preview_query.get_single_mut() else {
            continue;
        };

        redraw_road_preview(
            create_road_mesh_and_texture(&mut meshes, &mut images, event.road_data()),
            &mut preview_mesh,
            materials.get_mut(preview_material).unwrap(),
        );
    }
}
fn redraw_preview_on_active_road_modified(
    mut on_modified: EventReader<OnActiveRoadModified>,
    mut road_preview_query: Query<
        (&mut Handle<Mesh>, &Handle<StandardMaterial>),
        With<RoadPreview>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in on_modified.read() {
        let (mut preview_mesh, preview_material) = road_preview_query.get_single_mut().expect(
            "OnActiveRoadModified should only be called when a road preview already exists",
        );

        redraw_road_preview(
            create_road_mesh_and_texture(&mut meshes, &mut images, event.road_data()),
            &mut preview_mesh,
            materials.get_mut(preview_material).unwrap(),
        );
    }
}

fn redraw_road_preview(
    road_mesh_and_texture: (Handle<Mesh>, Handle<Image>),
    preview_mesh: &mut Handle<Mesh>,
    road_material: &mut StandardMaterial,
) {
    let (road_mesh, road_texture_image) = road_mesh_and_texture;

    *preview_mesh = road_mesh;
    road_material.base_color_texture = Some(road_texture_image);
}

fn create_road_mesh_and_texture(
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &mut ResMut<Assets<Image>>,
    road_data: &RoadData,
) -> (Handle<Mesh>, Handle<Image>) {
    let road_mesh = meshes.add(build_road_mesh(road_data.clone()).get_mesh());
    let road_texture_image = images.add(road_texture_from_road_data(road_data));

    (road_mesh, road_texture_image)
}

fn road_texture_from_road_data(road_data: &RoadData) -> Image {
    TextureBuilder::image_from_colors(
        road_data
            .components()
            .iter()
            .map(|component| component.color)
            .collect(),
    )
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

fn build_road_preview_bundle(
    road_mesh: Handle<Mesh>,
    road_material: Handle<StandardMaterial>,
) -> (RoadPreview, MaterialMeshBundle<StandardMaterial>) {
    (
        RoadPreview,
        PbrBundle {
            mesh: road_mesh,
            material: road_material,
            ..default()
        },
    )
}
