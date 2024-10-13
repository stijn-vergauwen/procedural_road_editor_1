pub mod active_road;
pub mod load;
pub mod persistance;
mod road_builder;
pub mod road_component;
pub mod road_data;
mod road_marking;
pub mod road_node;
mod road_preview;
pub mod road_section;
pub mod save;

use active_road::ActiveRoadPlugin;
use bevy::prelude::*;
use load::LoadRoadPlugin;
use road_marking::RoadMarkingPlugin;
use road_preview::RoadPreviewPlugin;
use road_section::RoadSectionPlugin;
use save::SaveRoadPlugin;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SaveRoadPlugin,
            LoadRoadPlugin,
            ActiveRoadPlugin,
            RoadPreviewPlugin,
            RoadMarkingPlugin,
            RoadSectionPlugin,
        ));
    }
}
