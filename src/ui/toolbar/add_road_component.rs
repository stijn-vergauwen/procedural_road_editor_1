use bevy::prelude::*;

use crate::{
    road::{
        active_road::active_road_events::new_road_component::OnNewRoadComponentRequested,
        road_component::RoadComponent,
    },
    ui::components::buttons::{ButtonAction, OnButtonPressed},
    GameRunningSet,
};

pub struct AddRoadComponentPlugin;

impl Plugin for AddRoadComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_add_road_component_button_pressed.in_set(GameRunningSet::SendCommands),
        );
    }
}

pub fn handle_add_road_component_button_pressed(
    mut on_pressed: EventReader<OnButtonPressed>,
    mut on_request: EventWriter<OnNewRoadComponentRequested>,
) {
    for _ in on_pressed
        .read()
        .filter(|event| event.is_action(ButtonAction::AddComponent))
    {
        on_request.send(OnNewRoadComponentRequested::new(RoadComponent::default()));
    }
}
