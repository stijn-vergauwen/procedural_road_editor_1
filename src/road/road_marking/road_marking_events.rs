pub mod hide_road_marking_config;
pub mod show_road_marking_config;

use bevy::prelude::*;
use hide_road_marking_config::OnHideRoadMarkingConfigRequested;
use show_road_marking_config::ShowRoadMarkingConfigPlugin;

pub struct RoadMarkingEventsPlugin;

impl Plugin for RoadMarkingEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShowRoadMarkingConfigPlugin)
            .add_event::<OnHideRoadMarkingConfigRequested>();
    }
}
