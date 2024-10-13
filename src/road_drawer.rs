mod gizmos;
mod road_being_drawn;
pub mod selected_road;

use bevy::prelude::*;
use gizmos::RoadDrawerGizmosPlugin;
use road_being_drawn::RoadBeingDrawnPlugin;
use selected_road::SelectedRoadPlugin;

use crate::{game_modes::GameMode, road::road_section::RequestedRoadSection};

pub struct RoadDrawerPlugin;

impl Plugin for RoadDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SelectedRoadPlugin,
            RoadDrawerGizmosPlugin,
            RoadBeingDrawnPlugin,
        ))
        .add_systems(OnEnter(GameMode::RoadDrawer), init_road_drawer)
        .add_systems(OnExit(GameMode::RoadDrawer), remove_road_drawer);
    }
}

#[derive(Resource, Default, Debug)]
pub struct RoadDrawer {
    section_being_drawn: Option<RequestedRoadSection>,
}

fn init_road_drawer(mut commands: Commands) {
    commands.init_resource::<RoadDrawer>();
}

fn remove_road_drawer(mut commands: Commands) {
    commands.remove_resource::<RoadDrawer>();
}
