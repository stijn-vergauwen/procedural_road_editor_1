mod gizmos;
mod road_being_drawn;
pub mod selected_road;

use bevy::prelude::*;
use gizmos::RoadDrawerGizmosPlugin;
use road_being_drawn::{RoadBeingDrawn, RoadBeingDrawnPlugin};
use selected_road::SelectedRoadPlugin;

pub struct RoadDrawerPlugin;

impl Plugin for RoadDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SelectedRoadPlugin,
            RoadDrawerGizmosPlugin,
            RoadBeingDrawnPlugin,
        ))
        .init_resource::<RoadDrawer>();
    }
}

// TODO: only have resource exist in RoadDrawer mode
#[derive(Resource, Default, Debug)]
pub struct RoadDrawer {
    road_being_drawn: Option<RoadBeingDrawn>,
}
