use bevy::prelude::*;

use crate::{game_modes::GameMode, GameRunningSet};

pub struct RoadDrawerToolPlugin;

impl Plugin for RoadDrawerToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(RoadDrawerTool::Drawer)
            .add_event::<OnRoadDrawerToolChangeRequested>()
            .add_event::<OnRoadDrawerToolChanged>()
            .add_systems(
                Update,
                handle_tool_change_requests
                    .in_set(GameRunningSet::HandleCommands)
                    .run_if(in_state(GameMode::RoadDrawer)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RoadDrawerTool {
    Drawer,
    Bulldozer,
}

#[derive(Event, Clone, Copy)]
pub struct OnRoadDrawerToolChangeRequested {
    tool: RoadDrawerTool,
}

impl OnRoadDrawerToolChangeRequested {
    pub fn new(tool: RoadDrawerTool) -> Self {
        Self { tool }
    }
}

#[derive(Event, Clone, Copy)]
pub struct OnRoadDrawerToolChanged {
    pub tool: RoadDrawerTool,
}

impl OnRoadDrawerToolChanged {
    pub fn new(tool: RoadDrawerTool) -> Self {
        Self { tool }
    }
}

fn handle_tool_change_requests(
    mut requests: EventReader<OnRoadDrawerToolChangeRequested>,
    mut on_changed: EventWriter<OnRoadDrawerToolChanged>,
    mut next_tool_state: ResMut<NextState<RoadDrawerTool>>,
) {
    for request in requests.read() {
        println!("Switch to tool: {:?}", request.tool);

        next_tool_state.set(request.tool);
        on_changed.send(OnRoadDrawerToolChanged::new(request.tool));
    }
}
