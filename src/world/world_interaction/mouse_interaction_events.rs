use bevy::prelude::*;

use crate::{utility::mouse_on_ui::MouseOnUi, GameRunningSet};

pub struct MouseInteractionEventsPlugin;

impl Plugin for MouseInteractionEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnMouseInteraction>().add_systems(
            Update,
            send_mouse_interaction_events.in_set(GameRunningSet::GetUserInput),
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionPhase {
    Started,
    Held,
    Released,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct OnMouseInteraction {
    pub phase: InteractionPhase,
    pub button: MouseButton,
    pub is_on_ui: bool,
}

impl OnMouseInteraction {
    pub fn new(phase: InteractionPhase, button: MouseButton, is_on_ui: bool) -> Self {
        Self {
            phase,
            button,
            is_on_ui,
        }
    }
}

fn send_mouse_interaction_events(
    mut on_interaction: EventWriter<OnMouseInteraction>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mouse_on_ui: Res<MouseOnUi>,
) {
    process_mouse_input_events(
        mouse_input.get_just_pressed(),
        &mut on_interaction,
        &mouse_on_ui,
        InteractionPhase::Started,
    );

    process_mouse_input_events(
        mouse_input.get_pressed(),
        &mut on_interaction,
        &mouse_on_ui,
        InteractionPhase::Held,
    );

    process_mouse_input_events(
        mouse_input.get_just_released(),
        &mut on_interaction,
        &mouse_on_ui,
        InteractionPhase::Released,
    );
}

fn process_mouse_input_events<'a>(
    events: impl ExactSizeIterator<Item = &'a MouseButton>,
    on_interaction: &mut EventWriter<OnMouseInteraction>,
    mouse_on_ui: &MouseOnUi,
    interaction_phase: InteractionPhase,
) {
    for mouse_button in events {
        on_interaction.send(OnMouseInteraction::new(
            interaction_phase,
            *mouse_button,
            mouse_on_ui.is_on_ui(),
        ));
    }
}
