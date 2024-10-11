use bevy::prelude::*;

use crate::GameRunningSet;

pub struct MouseInteractionEventsPlugin;

impl Plugin for MouseInteractionEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnMouseInteraction>().add_systems(
            Update,
            send_mouse_interaction_events.in_set(GameRunningSet::GetUserInput),
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InteractionPhase {
    Started,
    Held,
    Released,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct OnMouseInteraction {
    pub phase: InteractionPhase,
    pub button: MouseButton,
}

impl OnMouseInteraction {
    pub fn new(phase: InteractionPhase, button: MouseButton) -> Self {
        Self { phase, button }
    }
}

fn send_mouse_interaction_events(
    mut on_interaction: EventWriter<OnMouseInteraction>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    for mouse_button in mouse_input.get_just_pressed() {
        on_interaction.send(OnMouseInteraction::new(
            InteractionPhase::Started,
            *mouse_button,
        ));
    }

    for mouse_button in mouse_input.get_pressed() {
        on_interaction.send(OnMouseInteraction::new(
            InteractionPhase::Held,
            *mouse_button,
        ));
    }

    for mouse_button in mouse_input.get_just_released() {
        on_interaction.send(OnMouseInteraction::new(
            InteractionPhase::Released,
            *mouse_button,
        ));
    }
}
