mod load;
mod reorder_components;
mod road_builder;
mod road_component;
mod road_data;
mod road_editor;
mod save;

pub use load::OnLoadRoadRequested;
pub use reorder_components::OnReorderRoadComponentRequested;
pub use road_component::RoadComponent;
pub use road_data::RoadData;
pub use road_editor::{ActiveRoad, OnActiveRoadModified, RoadEditor};
pub use save::OnSaveRoadRequested;

use bevy::prelude::*;
use load::LoadRoadPlugin;
use reorder_components::ReorderRoadComponentsPlugin;
use road_builder::RoadBuilderPlugin;
use road_editor::RoadEditorPlugin;
use save::SaveRoadPlugin;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RoadBuilderPlugin,
            ReorderRoadComponentsPlugin,
            SaveRoadPlugin,
            LoadRoadPlugin,
            RoadEditorPlugin,
        ));
    }
}
