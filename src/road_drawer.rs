mod bulldozer;
mod gizmos;
mod road_drawer_tool;
mod section_being_drawn;
pub mod selected_road;

use bevy::prelude::*;
use bulldozer::BulldozerPlugin;
use gizmos::RoadDrawerGizmosPlugin;
use road_drawer_tool::RoadDrawerToolPlugin;
use section_being_drawn::{SectionBeingDrawn, SectionBeingDrawnPlugin};
use selected_road::SelectedRoadPlugin;

use crate::game_modes::GameMode;

pub struct RoadDrawerPlugin;

impl Plugin for RoadDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SelectedRoadPlugin,
            RoadDrawerGizmosPlugin,
            SectionBeingDrawnPlugin,
            BulldozerPlugin,
            RoadDrawerToolPlugin,
        ))
        .add_systems(OnEnter(GameMode::RoadDrawer), init_road_drawer)
        .add_systems(OnExit(GameMode::RoadDrawer), remove_road_drawer);
    }
}

#[derive(Resource, Default, Debug)]
pub struct RoadDrawer {
    section_being_drawn: Option<SectionBeingDrawn>,
}

fn init_road_drawer(mut commands: Commands) {
    commands.init_resource::<RoadDrawer>();
}

fn remove_road_drawer(mut commands: Commands) {
    commands.remove_resource::<RoadDrawer>();
}
