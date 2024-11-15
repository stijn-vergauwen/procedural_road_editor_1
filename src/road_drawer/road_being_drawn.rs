mod nearest_road_node;
pub mod section_being_drawn;
mod section_end_being_drawn;

use bevy::{math::InvalidDirectionError, prelude::*};
use nearest_road_node::NearestRoadNode;
use section_being_drawn::{
    CurvedSectionBeingDrawn, SectionBeingDrawn, SectionBeingDrawnError, SectionBeingDrawnVariant,
};
use section_end_being_drawn::SectionEndBeingDrawn;

use crate::{
    game_modes::GameMode,
    road::{road_node::RoadNode, road_section::road_section_builder::OnBuildRoadSectionRequested},
    utility::circular_arc::CircularArc,
    world::world_interaction::{
        interaction_target::OnWorldInteractionTargetUpdated,
        mouse_interaction_events::{InteractionPhase, OnMouseInteraction},
        WorldInteraction,
    },
    GameRunningSet,
};

use super::{road_drawer_tool::RoadDrawerTool, selected_road::SelectedRoad, RoadDrawer};

const MOUSE_BUTTON_TO_DRAW: MouseButton = MouseButton::Left;
const ROAD_NODE_SNAP_DISTANCE: f32 = 5.0;

// TODO: Curved section being drawn should set it's start direction to mouse position until mouse is pressed (instead of only setting it on mouse press)
//          - maybe add a boolean like "is_setting_start_direction" and update either start direction or curve arc in update system based on this bool
// TODO: right-clicking a curve being drawn should allow you to set a new start direction (instead of cancelling the whole section)

pub struct RoadBeingDrawnPlugin;

impl Plugin for RoadBeingDrawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                send_build_section_request_on_mouse_press.in_set(GameRunningSet::SendCommands),
                (
                    update_road_being_drawn_on_target_update,
                    set_curved_section_direction_on_mouse_press,
                    start_drawing_road_on_mouse_press,
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

// Systems

fn start_drawing_road_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
    road_node_query: Query<(Entity, &Transform), With<RoadNode>>,
    selected_road: Res<SelectedRoad>,
) {
    for _ in on_interaction.read().filter(|event| {
        event.button == MOUSE_BUTTON_TO_DRAW
            && event.phase == InteractionPhase::Started
            && !event.is_on_ui
    }) {
        if !selected_road.has_selected_road() || road_drawer.section_being_drawn.is_some() {
            continue;
        }

        let Some(interaction_target) = world_interaction.interaction_target() else {
            continue;
        };

        let target_position = interaction_target.position;
        let nearest_road_node = NearestRoadNode::find_from_position(
            &road_node_query,
            target_position,
            ROAD_NODE_SNAP_DISTANCE,
        );

        let road_section_end = SectionEndBeingDrawn::new(target_position, nearest_road_node);

        let section_being_drawn = SectionBeingDrawn {
            ends: [road_section_end; 2],
            variant: SectionBeingDrawnVariant::Curved(CurvedSectionBeingDrawn::empty()),
        };

        road_drawer.section_being_drawn = Some(section_being_drawn);
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
        let Some(section_being_drawn) = &mut road_drawer.section_being_drawn else {
            continue;
        };

        let nearest_road_node = NearestRoadNode::find_from_position(
            &road_node_query,
            interaction_target.position,
            ROAD_NODE_SNAP_DISTANCE,
        );

        let snapped_target_position = match nearest_road_node {
            Some(node) => node.position,
            None => interaction_target.position,
        };

        let snapped_start_position = section_being_drawn.start().snapped_position();

        match &mut section_being_drawn.variant {
            SectionBeingDrawnVariant::Straight => {
                section_being_drawn.ends[1] =
                    SectionEndBeingDrawn::new(interaction_target.position, nearest_road_node);
            }
            SectionBeingDrawnVariant::Curved(curved_section) => {
                if let Ok(start_direction) = curved_section.start_direction {
                    let inwards_start_transform =
                        Transform::from_translation(snapped_start_position)
                            .looking_to(start_direction, Dir3::Y);

                    curved_section.circular_arc = CircularArc::from_start_transform(
                        inwards_start_transform,
                        snapped_target_position,
                    )
                    .ok_or(SectionBeingDrawnError::InvalidCurve);

                    section_being_drawn.ends[1] =
                        SectionEndBeingDrawn::new(interaction_target.position, nearest_road_node);
                }
            }
        }
    }
}

fn set_curved_section_direction_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
) {
    for _ in on_interaction.read().filter(|event| {
        event.button == MOUSE_BUTTON_TO_DRAW
            && event.phase == InteractionPhase::Started
            && !event.is_on_ui
    }) {
        let Some(section_being_drawn) = &mut road_drawer.section_being_drawn else {
            continue;
        };

        let Some(interaction_target) = world_interaction.interaction_target() else {
            continue;
        };

        let snapped_start_position = section_being_drawn.start().snapped_position();

        if let SectionBeingDrawnVariant::Curved(curved_section) = &mut section_being_drawn.variant {
            if curved_section.start_direction.is_ok() {
                continue;
            }

            println!("Set start direction");

            curved_section.start_direction =
                get_direction_from_to(snapped_start_position, interaction_target.position)
                    .map_err(|_| SectionBeingDrawnError::InvalidSectionLength)
        }
    }
}

fn send_build_section_request_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut on_request_section: EventWriter<OnBuildRoadSectionRequested>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    for _ in on_interaction.read().filter(|event| {
        event.button == MOUSE_BUTTON_TO_DRAW
            && event.phase == InteractionPhase::Started
            && !event.is_on_ui
    }) {
        let Some(section_being_drawn) = &road_drawer.section_being_drawn else {
            continue;
        };

        let Ok(requested_section) = section_being_drawn.to_requested_road_section() else {
            continue;
        };

        on_request_section.send(OnBuildRoadSectionRequested::new(requested_section));
        road_drawer.section_being_drawn = None;
    }
}

fn cancel_road_on_right_click(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    for _ in on_interaction.read().filter(|event| {
        event.button == MouseButton::Right && event.phase == InteractionPhase::Started
    }) {
        road_drawer.section_being_drawn = None;
    }
}

fn cancel_road_when_leaving_drawer_tool(mut road_drawer: ResMut<RoadDrawer>) {
    road_drawer.section_being_drawn = None;
}

// Utility

fn get_direction_from_to(from: Vec3, to: Vec3) -> Result<Dir3, InvalidDirectionError> {
    Dir3::new(to - from)
}
