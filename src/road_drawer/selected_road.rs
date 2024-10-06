mod events;
mod ui;

use bevy::prelude::*;
pub use events::OnSelectRoadRequested;
use events::SelectedRoadEventsPlugin;
use ui::SelectedRoadUiPlugin;

use crate::road::road_data::RoadData;

pub struct SelectedRoadPlugin;

impl Plugin for SelectedRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SelectedRoadUiPlugin, SelectedRoadEventsPlugin))
            .insert_resource(SelectedRoad::default());
    }
}

#[derive(Resource, Debug, Default)]
pub struct SelectedRoad {
    selected_road: Option<RoadData>,
}

impl SelectedRoad {
    pub fn set_selected_road(&mut self, selected_road: RoadData) {
        self.selected_road = Some(selected_road);
    }
}
