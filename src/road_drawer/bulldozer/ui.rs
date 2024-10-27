use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    game_modes::GameMode,
    road_drawer::road_drawer_tool::{
        OnRoadDrawerToolChangeRequested, OnRoadDrawerToolChanged, RoadDrawerTool,
    },
    GameRunningSet,
};

const BUTTON_COLOR: Srgba = NEUTRAL_500;
const BUTTON_ACTIVE_COLOR: Srgba = YELLOW_500;

pub struct BulldozerUiPlugin;

impl Plugin for BulldozerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                send_request_on_button_pressed.in_set(GameRunningSet::SendCommands),
                change_button_color_on_tool_changed.in_set(GameRunningSet::UpdateEntities),
            )
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

fn change_button_color_on_tool_changed(
    mut on_changed: EventReader<OnRoadDrawerToolChanged>,
    mut button_query: Query<&mut BackgroundColor, With<BulldozerUiButton>>,
) {
    for event in on_changed.read() {
        let new_color = match event.tool {
            RoadDrawerTool::Bulldozer => BUTTON_ACTIVE_COLOR,
            _ => BUTTON_COLOR,
        };

        button_query.single_mut().0 = new_color.into();
    }
}
