use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    road::{
        road_node::{RequestedRoadNode, RoadNode},
        road_section::{
            road_section_builder::OnBuildRoadSectionRequested, RequestedRoadSection,
            RequestedRoadSectionEnd, RoadSectionShape,
        },
    },
    world::world_interaction::{
        interaction_target::{InteractionTarget, OnWorldInteractionTargetUpdated},
        mouse_interaction_events::{InteractionPhase, OnMouseInteraction},
        WorldInteraction,
    },
    GameRunningSet,
};

use super::{road_drawer_tool::RoadDrawerTool, selected_road::SelectedRoad, RoadDrawer};

const MOUSE_BUTTON_TO_DRAW: MouseButton = MouseButton::Left;
const ROAD_NODE_SNAP_DISTANCE: f32 = 5.0;

pub struct RoadBeingDrawnPlugin;

impl Plugin for RoadBeingDrawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                send_build_section_request_on_mouse_press.in_set(GameRunningSet::SendCommands),
                (
                    start_drawing_road_on_mouse_press,
                    update_road_being_drawn_on_target_update,
                    cancel_road_on_right_click,
                )
                    .chain()
                    .in_set(GameRunningSet::UpdateEntities),
            )
                .run_if(in_state(GameMode::RoadDrawer).and_then(in_state(RoadDrawerTool::Drawer))),
        )
        .add_systems(
            OnExit(RoadDrawerTool::Drawer),
            cancel_road_when_leaving_drawer_tool,
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RoadBeingDrawn {
    pub data: RequestedRoadSection,
    pub snapped_data: RequestedRoadSection,
}

fn start_drawing_road_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
    road_node_query: Query<(Entity, &Transform), With<RoadNode>>,
    selected_road: Res<SelectedRoad>,
) {
    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Started))
    {
        if !selected_road.has_selected_road() || road_drawer.road_being_drawn.is_some() {
            continue;
        }

        let Some(interaction_target) = world_interaction.interaction_target() else {
            continue;
        };

        let road_section_end = build_section_end(interaction_target);

        let snapped_road_section_end =
            snap_section_end_to_existing_node(road_section_end, &road_node_query);

        let road_being_drawn = RoadBeingDrawn {
            data: RequestedRoadSection {
                ends: [road_section_end; 2],
                shape: RoadSectionShape::Straight,
            },
            snapped_data: RequestedRoadSection {
                ends: [snapped_road_section_end, road_section_end],
                shape: RoadSectionShape::Straight,
            },
        };

        road_drawer.road_being_drawn = Some(road_being_drawn);
    }
}

fn update_road_being_drawn_on_target_update(
    mut on_target_updated: EventReader<OnWorldInteractionTargetUpdated>,
    mut road_drawer: ResMut<RoadDrawer>,
    road_node_query: Query<(Entity, &Transform), With<RoadNode>>,
) {
    for interaction_target in on_target_updated
        .read()
        .filter_map(|event| event.interaction_target)
    {
        let Some(road_being_drawn) = &mut road_drawer.road_being_drawn else {
            continue;
        };

        let road_section_end = build_section_end(interaction_target);

        road_being_drawn.data.ends[1] = road_section_end;
        road_being_drawn.snapped_data.ends[1] =
            snap_section_end_to_existing_node(road_section_end, &road_node_query)
    }
}

fn send_build_section_request_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut on_request_section: EventWriter<OnBuildRoadSectionRequested>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Started))
    {
        let Some(road_being_drawn) = road_drawer.road_being_drawn else {
            continue;
        };

        on_request_section.send(OnBuildRoadSectionRequested::new(
            road_being_drawn.snapped_data,
        ));
        road_drawer.road_being_drawn = None;
    }
}

fn cancel_road_on_right_click(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    for _ in on_interaction.read().filter(|event| {
        event.button == MouseButton::Right && event.phase == InteractionPhase::Started
    }) {
        road_drawer.road_being_drawn = None;
    }
}

fn cancel_road_when_leaving_drawer_tool(mut road_drawer: ResMut<RoadDrawer>) {
    road_drawer.road_being_drawn = None;
}

// Utility

fn filter_mouse_interaction(event: &&OnMouseInteraction, phase: InteractionPhase) -> bool {
    event.button == MOUSE_BUTTON_TO_DRAW && event.phase == phase && !event.is_on_ui
}

// Snapping utility

#[derive(Clone, Copy)]
struct NearestRoadNode {
    position: Vec3,
    entity: Entity,
}

impl NearestRoadNode {
    fn to_requested_road_node(&self) -> RequestedRoadNode {
        RequestedRoadNode::new(self.position, Some(self.entity))
    }
}

fn build_section_end(interaction_target: InteractionTarget) -> RequestedRoadSectionEnd {
    RequestedRoadSectionEnd::new(
        RequestedRoadNode::new(interaction_target.point, None),
        Dir3::X, // TODO: calculate direction
    )
}

fn snap_section_end_to_existing_node(
    mut requested_road_section_end: RequestedRoadSectionEnd,
    road_node_query: &Query<(Entity, &Transform), With<RoadNode>>,
) -> RequestedRoadSectionEnd {
    requested_road_section_end.road_node =
        snap_road_node_to_existing_node(requested_road_section_end.road_node, road_node_query);
    requested_road_section_end
}

fn snap_road_node_to_existing_node(
    requested_road_node: RequestedRoadNode,
    road_node_query: &Query<(Entity, &Transform), With<RoadNode>>,
) -> RequestedRoadNode {
    match find_road_node_nearest_to_point(
        road_node_query,
        requested_road_node.position,
        ROAD_NODE_SNAP_DISTANCE,
    ) {
        Some(nearest_node) => nearest_node.to_requested_road_node(),
        None => requested_road_node,
    }
}

fn find_road_node_nearest_to_point(
    road_node_query: &Query<(Entity, &Transform), With<RoadNode>>,
    point: Vec3,
    max_distance: f32,
) -> Option<NearestRoadNode> {
    let (node_entity, node_transform, node_distance) = road_node_query
        .iter()
        .map(|(node_entity, node_transform)| {
            (
                node_entity,
                node_transform,
                node_transform.translation.distance(point),
            )
        })
        .min_by(|(_, _, distance_a), (_, _, distance_b)| distance_a.total_cmp(&distance_b))?;

    (node_distance < max_distance).then_some(NearestRoadNode {
        position: node_transform.translation,
        entity: node_entity,
    })
}
