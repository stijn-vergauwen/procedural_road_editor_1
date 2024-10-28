mod bulldozer;
mod gizmos;
mod road_being_drawn;
mod road_drawer_tool;
pub mod selected_road;

use bevy::prelude::*;
use bulldozer::BulldozerPlugin;
use gizmos::RoadDrawerGizmosPlugin;
use road_being_drawn::{RoadBeingDrawn, RoadBeingDrawnPlugin};
use road_drawer_tool::RoadDrawerToolPlugin;
use selected_road::SelectedRoadPlugin;

use crate::game_modes::GameMode;

pub struct RoadDrawerPlugin;

impl Plugin for RoadDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SelectedRoadPlugin,
            RoadDrawerGizmosPlugin,
            RoadBeingDrawnPlugin,
            BulldozerPlugin,
            RoadDrawerToolPlugin,
        ))
        .add_systems(OnEnter(GameMode::RoadDrawer), init_road_drawer)
        .add_systems(OnExit(GameMode::RoadDrawer), remove_road_drawer);
    }
}

#[derive(Resource, Default, Debug)]
pub struct RoadDrawer {
    road_being_drawn: Option<RoadBeingDrawn>,
}

fn init_road_drawer(mut commands: Commands) {
    commands.init_resource::<RoadDrawer>();
}

fn remove_road_drawer(mut commands: Commands) {
    commands.remove_resource::<RoadDrawer>();
}
