use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    road::{
        road_node::RequestedRoadNode,
        road_section::{road_section_builder::OnBuildRoadSectionRequested, RequestedRoadSection},
    },
    world::world_interaction::{
        interaction_target::OnWorldInteractionTargetUpdated,
        mouse_interaction_events::{InteractionPhase, OnMouseInteraction},
        WorldInteraction,
    },
    GameRunningSet,
};

use super::RoadDrawer;

const MOUSE_BUTTON_TO_DRAW: MouseButton = MouseButton::Left;

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
                    stop_drawing_road_on_right_click,
                )
                    .chain()
                    .in_set(GameRunningSet::UpdateEntities),
            )
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn start_drawing_road_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
) {
    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Started))
    {
        let Some(interaction_target) = world_interaction.interaction_target() else {
            return;
        };
    
        let interaction_position = interaction_target.point;

        road_drawer.section_being_drawn = Some(RequestedRoadSection {
            start: RequestedRoadNode::new(interaction_position, None),
            end: RequestedRoadNode::new(interaction_position, None),
        });
    }
}

fn update_road_being_drawn_on_target_update(
    mut on_target_updated: EventReader<OnWorldInteractionTargetUpdated>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    for interaction_target in on_target_updated
        .read()
        .filter_map(|event| event.interaction_target)
    {
        let Some(section_being_drawn) = &mut road_drawer.section_being_drawn else {
            return;
        };
    
        section_being_drawn.end.position = interaction_target.point;
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
        let Some(section_being_drawn) = road_drawer.section_being_drawn else {
            return;
        };

        on_request_section.send(OnBuildRoadSectionRequested::new(section_being_drawn));
        road_drawer.section_being_drawn = None;
    }
}

fn stop_drawing_road_on_right_click(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    for _ in on_interaction.read().filter(|event| {
        event.button == MouseButton::Right && event.phase == InteractionPhase::Started
    }) {
        road_drawer.section_being_drawn = None;
    }
}

// TODO: if all 'desired_on_ui' values are the same, remove param etc
fn filter_mouse_interaction(
    event: &&OnMouseInteraction,
    phase: InteractionPhase,
) -> bool {
    event.button == MOUSE_BUTTON_TO_DRAW && event.phase == phase && !event.is_on_ui
}
