pub mod active_road;
mod load;
mod road_builder;
mod road_component;
mod road_data;
mod road_marking;
mod road_preview;
mod save;

pub use active_road::ActiveRoad;
pub use load::OnLoadRoadRequested;
pub use road_component::RoadComponent;
pub use road_data::RoadData;
use road_marking::RoadMarkingPlugin;
pub use save::OnSaveRoadRequested;
pub use road_component::RoadComponentField;

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
            RoadMarkingPlugin,
        ));
    }
}
