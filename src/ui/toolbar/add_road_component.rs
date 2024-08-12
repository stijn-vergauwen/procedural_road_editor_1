use bevy::prelude::*;

use crate::{
    road::{active_road::new_road_component::OnNewRoadComponentRequested, RoadComponent},
    GameRunningSet,
};

use super::ToolbarAction;

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
    mut on_request: EventWriter<OnNewRoadComponentRequested>,
    button_query: Query<(&Interaction, &ToolbarAction), Changed<Interaction>>,
) {
    for (interaction, action) in button_query.iter() {
        if *interaction == Interaction::Pressed && *action == ToolbarAction::AddComponent {
            on_request.send(OnNewRoadComponentRequested::new(RoadComponent::default()));
        }
    }
}
