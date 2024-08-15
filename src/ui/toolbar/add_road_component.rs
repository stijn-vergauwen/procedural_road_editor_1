use bevy::prelude::*;

use crate::{
    road::{active_road::new_road_component::OnNewRoadComponentRequested, RoadComponent},
    ui::buttons::{ButtonAction, OnButtonPressed},
    GameRunningSet,
};

use super::RoadComponentsList;

pub struct AddRoadComponentPlugin;

impl Plugin for AddRoadComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_add_road_component_button_pressed.in_set(GameRunningSet::GetUserInput),
        );
    }
}

pub fn handle_add_road_component_button_pressed(
    mut on_pressed: EventReader<OnButtonPressed>,
    mut on_request: EventWriter<OnNewRoadComponentRequested>,
    component_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for _ in on_pressed
        .read()
        .filter(|event| event.is_action(ButtonAction::AddComponent))
    {
        let component_list_entity = component_list_query.single();
        on_request.send(OnNewRoadComponentRequested::new(
            RoadComponent::default(),
            component_list_entity,
        ));
    }
}
