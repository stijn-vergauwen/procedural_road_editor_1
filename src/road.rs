mod active_road;
mod load;
mod road_builder;
mod road_component;
mod road_data;
mod road_preview;
mod save;

pub use active_road::{
    reorder_road_components::OnReorderRoadComponentRequested, ActiveRoad, OnActiveRoadModified,
};
pub use load::OnLoadRoadRequested;
pub use road_component::RoadComponent;
pub use road_data::RoadData;
pub use save::OnSaveRoadRequested;

use active_road::{reorder_road_components::ReorderRoadComponentsPlugin, RoadEditorPlugin};
use bevy::prelude::*;
use load::LoadRoadPlugin;
use road_preview::RoadPreviewPlugin;
use save::SaveRoadPlugin;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ReorderRoadComponentsPlugin,
            SaveRoadPlugin,
            LoadRoadPlugin,
            RoadEditorPlugin,
            RoadPreviewPlugin,
        ));
    }
}
