mod selected_road;

use bevy::prelude::*;
use selected_road::SelectedRoadPlugin;

pub struct RoadDrawerPlugin;

impl Plugin for RoadDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectedRoadPlugin);
    }
}
