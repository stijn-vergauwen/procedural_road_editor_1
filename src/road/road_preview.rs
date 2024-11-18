use bevy::prelude::*;

use crate::{game_modes::GameMode, GameRunningSet};

use super::{
    active_road::{
        active_road_events::{
            new_road_component::OnRoadComponentAdded,
            road_component_change::OnRoadComponentChanged,
            road_component_deletion::OnRoadComponentDeleted,
            road_component_reorder::OnRoadComponentReordered, OnActiveRoadSet,
        },
        ActiveRoad,
    },
    road_builder::RoadBuilder,
    road_data::RoadData,
    road_node::RequestedRoadNode,
    road_section::{RequestedRoadSection, RequestedRoadSectionEnd, RoadSectionVariant},
};

const PREVIEW_ROAD_LENGTH: f32 = 20.0;

pub struct RoadPreviewPlugin;

impl Plugin for RoadPreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                redraw_preview_on_active_road_set,
                spawn_preview_on_active_road_set,
                redraw_preview_on_road_component_added,
                redraw_preview_on_road_component_changed,
                redraw_preview_on_road_component_reordered,
                redraw_preview_on_road_component_deleted,
            )
                .chain()
                .in_set(GameRunningSet::UpdateEntities)
                .run_if(in_state(GameMode::RoadEditor)),
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

fn redraw_preview_on_road_component_added(
    mut on_added: EventReader<OnRoadComponentAdded>,
    mut road_preview_query: Query<
        (&mut Handle<Mesh>, &Handle<StandardMaterial>),
        With<RoadPreview>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in on_added.read() {
        let (mut preview_mesh, preview_material) = road_preview_query
            .get_single_mut()
            .expect("This event should only be sent when a road preview already exists");

        redraw_road_preview(
            create_road_mesh_and_texture(&mut meshes, &mut images, event.new_road_data()),
            &mut preview_mesh,
            materials.get_mut(preview_material).unwrap(),
        );
    }
}

fn redraw_preview_on_road_component_changed(
    mut on_changed: EventReader<OnRoadComponentChanged>,
    mut road_preview_query: Query<
        (&mut Handle<Mesh>, &Handle<StandardMaterial>),
        With<RoadPreview>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in on_changed.read() {
        let (mut preview_mesh, preview_material) = road_preview_query
            .get_single_mut()
            .expect("This event should only be sent when a road preview already exists");

        redraw_road_preview(
            create_road_mesh_and_texture(&mut meshes, &mut images, event.new_road_data()),
            &mut preview_mesh,
            materials.get_mut(preview_material).unwrap(),
        );
    }
}

fn redraw_preview_on_road_component_reordered(
    mut on_reordered: EventReader<OnRoadComponentReordered>,
    mut road_preview_query: Query<
        (&mut Handle<Mesh>, &Handle<StandardMaterial>),
        With<RoadPreview>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in on_reordered.read() {
        let (mut preview_mesh, preview_material) = road_preview_query
            .get_single_mut()
            .expect("This event should only be sent when a road preview already exists");

        redraw_road_preview(
            create_road_mesh_and_texture(&mut meshes, &mut images, event.new_road_data()),
            &mut preview_mesh,
            materials.get_mut(preview_material).unwrap(),
        );
    }
}

fn redraw_preview_on_road_component_deleted(
    mut on_deleted: EventReader<OnRoadComponentDeleted>,
    mut road_preview_query: Query<
        (&mut Handle<Mesh>, &Handle<StandardMaterial>),
        With<RoadPreview>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in on_deleted.read() {
        let (mut preview_mesh, preview_material) = road_preview_query
            .get_single_mut()
            .expect("This event should only be sent when a road preview already exists");

        redraw_road_preview(
            create_road_mesh_and_texture(&mut meshes, &mut images, event.new_road_data()),
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
    meshes: &mut Assets<Mesh>,
    images: &mut Assets<Image>,
    road_data: &RoadData,
) -> (Handle<Mesh>, Handle<Image>) {
    let requested_section = RequestedRoadSection {
        ends: [
            RequestedRoadSectionEnd {
                road_node: RequestedRoadNode::new(Vec3::Z * PREVIEW_ROAD_LENGTH / 2.0, None),
                direction: Dir3::Z,
            },
            RequestedRoadSectionEnd {
                road_node: RequestedRoadNode::new(Vec3::NEG_Z * PREVIEW_ROAD_LENGTH / 2.0, None),
                direction: Dir3::NEG_Z,
            },
        ],
        variant: RoadSectionVariant::Straight,
    };

    let mut road_builder = RoadBuilder::new();
    road_builder.build_from_requested_section(&requested_section, road_data);

    let road_mesh = meshes.add(road_builder.get_mesh());
    let road_texture_image = images.add(road_builder.get_texture_image());

    (road_mesh, road_texture_image)
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
