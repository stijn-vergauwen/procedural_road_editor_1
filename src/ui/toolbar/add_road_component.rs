use bevy::prelude::*;

use crate::{
    road::{OnNewRoadComponentRequested, RoadComponent},
    GameRunningSet,
};

pub struct AddRoadComponentPlugin;

impl Plugin for AddRoadComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_add_road_component_button_pressed.in_set(GameRunningSet::GetUserInput),
        );
    }
}

#[derive(Component)]
pub struct AddRoadComponentButton;

pub fn handle_add_road_component_button_pressed(
    mut on_request: EventWriter<OnNewRoadComponentRequested>,
    button_query: Query<&Interaction, (With<AddRoadComponentButton>, Changed<Interaction>)>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            on_request.send(OnNewRoadComponentRequested::new(RoadComponent::default()));
        }
    }
}
