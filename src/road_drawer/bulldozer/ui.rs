use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    road_drawer::road_drawer_tool::{OnRoadDrawerToolChangeRequested, RoadDrawerTool},
    GameRunningSet,
};

pub struct BulldozerUiPlugin;

impl Plugin for BulldozerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_request_on_button_pressed
                .in_set(GameRunningSet::SendCommands)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

#[derive(Component, Clone, Copy)]
pub struct BulldozerUiButton;

fn send_request_on_button_pressed(
    mut on_request: EventWriter<OnRoadDrawerToolChangeRequested>,
    button_query: Query<&Interaction, (With<BulldozerUiButton>, Changed<Interaction>)>,
    current_tool: Res<State<RoadDrawerTool>>,
) {
    for _ in button_query
        .iter()
        .filter(|interaction| **interaction == Interaction::Pressed)
    {
        let new_value = match current_tool.get() {
            RoadDrawerTool::Bulldozer => RoadDrawerTool::Drawer,
            _ => RoadDrawerTool::Bulldozer,
        };

        on_request.send(OnRoadDrawerToolChangeRequested::new(new_value));
    }
}
