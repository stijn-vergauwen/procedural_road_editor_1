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
        app.add_event::<OnRoadSectionDrawn>().add_systems(
            Update,
            (
                start_drawing_road_on_mouse_press,
                update_road_being_drawn_on_mouse_drag,
                send_road_section_drawn_event_on_mouse_release,
                reset_section_being_drawn_on_esc,
            )
                .chain()
                .in_set(GameRunningSet::UpdateEntities)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RoadSectionBeingDrawn {
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

#[derive(Event, Clone, Copy, Debug)]
pub struct OnRoadSectionDrawn {
    pub section_being_drawn: RoadSectionBeingDrawn,
}

impl OnRoadSectionDrawn {
    pub fn new(section_being_drawn: RoadSectionBeingDrawn) -> Self {
        Self {
            section_being_drawn,
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

        road_drawer.section_being_drawn = Some(RoadSectionBeingDrawn {
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

    let Some(section_being_drawn) = &mut road_drawer.section_being_drawn else {
        return;
    };

    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Held, Some(false)))
    {
        section_being_drawn.end.position = interaction_target.point;
    }
}

fn send_road_section_drawn_event_on_mouse_release(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut on_drawn: EventWriter<OnRoadSectionDrawn>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    let Some(section_being_drawn) = road_drawer.section_being_drawn else {
        return;
    };

    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Released, Some(false)))
    {
        on_drawn.send(OnRoadSectionDrawn::new(section_being_drawn));
        road_drawer.section_being_drawn = None;
    }
}

fn reset_section_being_drawn_on_esc(
    mut road_drawer: ResMut<RoadDrawer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        road_drawer.section_being_drawn = None;
    }
}

// TODO: if all 'desired_on_ui' values are the same, remove param etc
fn filter_mouse_interaction(
    event: &&OnMouseInteraction,
    phase: InteractionPhase,
    desired_on_ui: Option<bool>,
) -> bool {
    let has_correct_on_ui = match desired_on_ui {
        Some(should_be_on_ui) => should_be_on_ui == event.is_on_ui,
        None => true,
    };

    event.button == MOUSE_BUTTON_TO_DRAW && event.phase == phase && has_correct_on_ui
}
