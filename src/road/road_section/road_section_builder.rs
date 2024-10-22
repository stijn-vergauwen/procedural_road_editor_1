use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    game_modes::GameMode,
    road::{road_data::RoadData, road_node::road_node_builder::get_or_build_road_node},
    road_drawer::selected_road::SelectedRoad,
    GameRunningSet,
};

use super::{
    calculate_road_section_size, calculate_road_section_transform, RequestedRoadSection,
    RoadSection,
};

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
        Self { requested_section }
    }
}

fn build_road_sections_on_request(
    mut requests: EventReader<OnBuildRoadSectionRequested>,
    mut commands: Commands,
    selected_road: Res<SelectedRoad>,
) {
    for request in requests.read() {
        let selected_road_design = selected_road
            .selected_road()
            .expect("Requests should only be possible to send when a road is selected");

        let road_section = get_requested_road_section(
            &request.requested_section,
            &mut commands,
            selected_road_design.clone(),
        );

        let start_node_position = request.requested_section.start.position;
        let end_node_position = request.requested_section.end.position;

        let section_transform =
            calculate_road_section_transform(start_node_position, end_node_position);
        let spatial_bundle = SpatialBundle::from_transform(section_transform);

        let half_section_size = calculate_road_section_size(
            selected_road_design,
            start_node_position,
            end_node_position,
        ) / 2.0;
        let collider = Collider::cuboid(
            half_section_size.x,
            half_section_size.y,
            half_section_size.z,
        );

        commands.spawn((road_section, spatial_bundle, collider));
    }
}

fn get_requested_road_section(
    requested_section: &RequestedRoadSection,
    commands: &mut Commands,
    road_design: RoadData,
) -> RoadSection {
    let start_node_entity = get_or_build_road_node(commands, requested_section.start);
    let end_node_entity = get_or_build_road_node(commands, requested_section.end);

    RoadSection::new(start_node_entity, end_node_entity, road_design)
}
