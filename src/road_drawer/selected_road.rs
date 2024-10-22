mod events;
mod ui;

use bevy::prelude::*;
pub use events::OnSelectRoadRequested;
use events::SelectedRoadEventsPlugin;
use ui::SelectedRoadUiPlugin;

use crate::{game_modes::GameMode, road::road_data::RoadData};

pub struct SelectedRoadPlugin;

impl Plugin for SelectedRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SelectedRoadUiPlugin, SelectedRoadEventsPlugin))
            .add_systems(OnEnter(GameMode::RoadDrawer), init_selected_road)
            .add_systems(OnExit(GameMode::RoadDrawer), remove_selected_road);
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

    pub fn has_selected_road(&self) -> bool {
        self.selected_road.is_some()
    }

    pub fn selected_road(&self) -> Option<&RoadData> {
        self.selected_road.as_ref()
    }
}

fn init_selected_road(mut commands: Commands) {
    commands.init_resource::<SelectedRoad>();
}

fn remove_selected_road(mut commands: Commands) {
    commands.remove_resource::<SelectedRoad>();
}
