mod active_road;
mod load;
mod road_builder;
mod road_component;
mod road_data;
mod road_preview;
mod save;

pub use active_road::{
    new_road_component::OnNewRoadComponentRequested,
    road_component_change::OnRoadComponentChangeRequested,
    road_component_deletion::OnRoadComponentDeletionRequested,
    road_component_reorder::OnRoadComponentReorderRequested, ActiveRoad, OnActiveRoadModified,
};
pub use load::OnLoadRoadRequested;
pub use road_component::RoadComponent;
pub use road_data::RoadData;
pub use save::OnSaveRoadRequested;

use active_road::ActiveRoadPlugin;
use bevy::prelude::*;
use load::LoadRoadPlugin;
use road_preview::RoadPreviewPlugin;
use save::SaveRoadPlugin;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SaveRoadPlugin,
            LoadRoadPlugin,
            ActiveRoadPlugin,
            RoadPreviewPlugin,
        ));
    }
}
