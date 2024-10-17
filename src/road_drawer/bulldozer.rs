use bevy::prelude::*;

use crate::{game_modes::GameMode, GameRunningSet};

use super::RoadDrawerTool;

pub struct BulldozerPlugin;

impl Plugin for BulldozerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            switch_to_bulldozer_on_b_key
                .in_set(GameRunningSet::GetUserInput)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn switch_to_bulldozer_on_b_key(
    current_tool: Res<State<RoadDrawerTool>>,
    mut next_tool: ResMut<NextState<RoadDrawerTool>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        let new_value = match current_tool.get() {
            RoadDrawerTool::Drawer => RoadDrawerTool::Bulldozer,
            RoadDrawerTool::Bulldozer => RoadDrawerTool::Drawer,
        };

        println!("Switch to tool: {:?}", new_value);

        next_tool.set(new_value);
    }
}
