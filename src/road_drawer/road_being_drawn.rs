use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    world::world_interaction::{
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
                start_drawing_road_on_mouse_press,
                update_road_being_drawn_on_mouse_drag,
            )
                .in_set(GameRunningSet::UpdateEntities)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RoadBeingDrawn {
    pub start: RoadNodeBeingDrawn,
    pub end: RoadNodeBeingDrawn,
}

#[derive(Clone, Copy, Debug)]
pub struct RoadNodeBeingDrawn {
    pub position: Vec3,
    /// Entity of an existing RoadNode if the road starts / ends on an existing node, otherwise None.
    pub existing_node_entity: Option<Entity>,
}

impl RoadNodeBeingDrawn {
    fn new(position: Vec3, existing_node_entity: Option<Entity>) -> Self {
        Self {
            position,
            existing_node_entity,
        }
    }
}

fn start_drawing_road_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
) {
    let Some(interaction_target) = world_interaction.interaction_target() else {
        return;
    };

    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Started, Some(false)))
    {
        let interaction_position = interaction_target.point;

        road_drawer.road_being_drawn = Some(RoadBeingDrawn {
            start: RoadNodeBeingDrawn::new(interaction_position, None),
            end: RoadNodeBeingDrawn::new(interaction_position, None),
        });
    }
}

fn update_road_being_drawn_on_mouse_drag(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
) {
    let Some(interaction_target) = world_interaction.interaction_target() else {
        return;
    };

    let Some(road_being_drawn) = &mut road_drawer.road_being_drawn else {
        return;
    };

    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Held, None))
    {
        let interaction_position = interaction_target.point;
        road_being_drawn.end.position = interaction_position;
    }
}

fn filter_mouse_interaction(event: &&OnMouseInteraction, phase: InteractionPhase, desired_on_ui: Option<bool>) -> bool {
    let has_correct_on_ui = match desired_on_ui {
        Some(should_be_on_ui) => should_be_on_ui == event.is_on_ui,
        None => true,
    };

    event.button == MOUSE_BUTTON_TO_DRAW && event.phase == phase && has_correct_on_ui
}
