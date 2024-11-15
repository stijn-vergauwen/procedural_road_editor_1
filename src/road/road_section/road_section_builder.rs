use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    game_modes::GameMode,
    road::{
        road_builder::RoadBuilder, road_data::RoadData,
        road_node::road_node_builder::get_or_build_road_node,
    },
    road_drawer::selected_road::SelectedRoad,
    GameRunningSet,
};

use super::{
    calculate_road_section_size, calculate_road_section_transform, RequestedRoadSection,
    RoadSection, RoadSectionEnd,
};

// TODO: 
// TODO: replace the cuboid collider with a mesh collider from the RoadBuilder

pub struct RoadSectionBuilderPlugin;

impl Plugin for RoadSectionBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnBuildRoadSectionRequested>().add_systems(
            Update,
            build_road_sections_on_request
                .in_set(GameRunningSet::HandleCommands)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

#[derive(Event)]
pub struct OnBuildRoadSectionRequested {
    requested_section: RequestedRoadSection,
}

impl OnBuildRoadSectionRequested {
    pub fn new(requested_section: RequestedRoadSection) -> Self {
        // TODO: assert that straight sections have to have their ends in opposite directions
        // TODO: assert that curved sections can't have their ends in opposite directions
        Self { requested_section }
    }
}

fn build_road_sections_on_request(
    mut requests: EventReader<OnBuildRoadSectionRequested>,
    mut commands: Commands,
    selected_road: Res<SelectedRoad>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut image_assets: ResMut<Assets<Image>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    for request in requests.read() {
        let requested_section = &request.requested_section;
        let selected_road_design = selected_road
            .selected_road()
            .expect("Requests should only be possible to send when a road is selected");

        let road_section = get_requested_road_section(
            requested_section,
            &mut commands,
            selected_road_design.clone(),
        );

        let road_section_size = calculate_road_section_size(
            selected_road_design,
            requested_section.start().road_node.position,
            requested_section.end().road_node.position,
        );

        let pbr_bundle = build_road_section_pbr_bundle(
            &mut mesh_assets,
            &mut image_assets,
            &mut material_assets,
            requested_section,
            selected_road_design,
            road_section_size.z,
        );

        let collider = get_road_section_collider(road_section_size);

        commands.spawn((road_section, pbr_bundle, collider));
    }
}

fn get_requested_road_section(
    requested_section: &RequestedRoadSection,
    commands: &mut Commands,
    road_design: RoadData,
) -> RoadSection {
    let ends = requested_section.ends.map(|end| {
        RoadSectionEnd::new(
            get_or_build_road_node(commands, end.road_node),
            end.direction,
        )
    });

    RoadSection::new(ends, requested_section.variant, road_design)
}

fn build_road_section_pbr_bundle(
    mesh_assets: &mut Assets<Mesh>,
    image_assets: &mut Assets<Image>,
    material_assets: &mut Assets<StandardMaterial>,
    requested_section: &RequestedRoadSection,
    road_data: &RoadData,
    road_length: f32,
) -> PbrBundle {
    let mut road_builder = RoadBuilder::new(road_length);
    road_builder.build_from_road_data(road_data);

    let road_mesh_handle = mesh_assets.add(road_builder.get_mesh());

    let road_texture_image_handle = image_assets.add(road_builder.get_texture_image());
    let road_material_handle = material_assets.add(StandardMaterial {
        base_color_texture: Some(road_texture_image_handle),
        perceptual_roughness: 0.7,
        ..default()
    });

    PbrBundle {
        mesh: road_mesh_handle,
        material: road_material_handle,
        transform: calculate_road_section_transform(
            requested_section.start().road_node.position,
            requested_section.end().road_node.position,
        ),
        ..default()
    }
}

fn get_road_section_collider(road_section_size: Vec3) -> Collider {
    let half_size = road_section_size / 2.0;

    Collider::cuboid(half_size.x, half_size.y, half_size.z)
}
