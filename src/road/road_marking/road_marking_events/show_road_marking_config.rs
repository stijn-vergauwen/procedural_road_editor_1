use bevy::prelude::*;

use crate::{
    ui::components::buttons::{ButtonAction, OnButtonPressed},
    GameRunningSet,
};

pub struct ShowRoadMarkingConfigPlugin;

impl Plugin for ShowRoadMarkingConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnShowRoadMarkingConfigRequested>()
            .add_systems(
                Update,
                send_request_on_button_pressed.in_set(GameRunningSet::SendCommands),
            );
    }
}

#[derive(Event, Clone, Copy)]
pub struct OnShowRoadMarkingConfigRequested;

fn send_request_on_button_pressed(
    mut on_pressed: EventReader<OnButtonPressed>,
    mut on_request: EventWriter<OnShowRoadMarkingConfigRequested>,
) {
    for _ in on_pressed
        .read()
        .filter(|event| event.is_action(ButtonAction::ShowRoadMarkingConfig))
    {
        on_request.send(OnShowRoadMarkingConfigRequested);
    }
}
