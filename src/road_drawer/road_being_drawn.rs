use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    utility::mouse_on_ui::MouseOnUi,
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
            start_drawing_road_on_mouse_interaction
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

fn start_drawing_road_on_mouse_interaction(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
    mouse_on_ui: Res<MouseOnUi>,
) {
    if mouse_on_ui.is_on_ui() {
        return;
    }

    let Some(interaction_target) = world_interaction.interaction_target() else {
        return;
    };

    for _ in on_interaction.read().filter(|event| {
        event.button == MOUSE_BUTTON_TO_DRAW && event.phase == InteractionPhase::Started
    }) {
        let interaction_position = interaction_target.point;

        road_drawer.road_being_drawn = Some(RoadBeingDrawn {
            start: RoadNodeBeingDrawn::new(interaction_position, None),
            end: RoadNodeBeingDrawn::new(interaction_position, None),
        });
    }
}
