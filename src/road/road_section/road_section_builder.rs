use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    game_modes::GameMode, road::road_node::road_node_builder::get_or_build_road_node,
    GameRunningSet,
};

use super::{calculate_road_section_transform, RequestedRoadSection, RoadSection};

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
) {
    for request in requests.read() {
        let start_node = request.requested_section.start;
        let end_node = request.requested_section.end;

        let road_section = get_requested_road_section(&request.requested_section, &mut commands);

        let section_transform =
            calculate_road_section_transform(start_node.position, end_node.position);

        let spatial_bundle = SpatialBundle::from_transform(section_transform);
        let collider = Collider::cuboid(0.5, 0.5, 0.5);

        commands.spawn((road_section, spatial_bundle, collider));
    }
}

fn get_requested_road_section(
    requested_section: &RequestedRoadSection,
    commands: &mut Commands,
) -> RoadSection {
    let start_node_entity = get_or_build_road_node(commands, requested_section.start);
    let end_node_entity = get_or_build_road_node(commands, requested_section.end);

    RoadSection::new(start_node_entity, end_node_entity)
}
