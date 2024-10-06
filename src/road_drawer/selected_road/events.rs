use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    road::{persistance::load_road_data, road_data::RoadData},
    GameRunningSet,
};

use super::SelectedRoad;

pub struct SelectedRoadEventsPlugin;

impl Plugin for SelectedRoadEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSelectRoadRequested>()
            .add_event::<OnRoadSelected>()
            .add_systems(
                Update,
                handle_select_requests
                    .in_set(GameRunningSet::HandleCommands)
                    .run_if(in_state(GameMode::RoadDrawer)),
            );
    }
}

#[derive(Event)]
pub struct OnSelectRoadRequested {
    road_name: String,
}

impl OnSelectRoadRequested {
    pub fn new(road_name: String) -> Self {
        Self { road_name }
    }
}

#[derive(Event, Clone)]
pub struct OnRoadSelected {
    pub road_data: RoadData,
}

impl OnRoadSelected {
    pub fn new(road_data: RoadData) -> Self {
        Self { road_data }
    }
}

fn handle_select_requests(
    mut requests: EventReader<OnSelectRoadRequested>,
    mut on_selected: EventWriter<OnRoadSelected>,
    mut selected_road: ResMut<SelectedRoad>,
) {
    for request in requests.read() {
        let road_data = load_road_data(&request.road_name).expect("RoadData should load from file");

        selected_road.set_selected_road(road_data.clone());
        on_selected.send(OnRoadSelected::new(road_data));
    }
}
