mod ui;

use bevy::prelude::*;
use ui::SelectedRoadUiPlugin;

use crate::{
    game_modes::GameMode,
    road::{persistance::load_road_data, road_data::RoadData},
    GameRunningSet,
};

pub struct SelectedRoadPlugin;

impl Plugin for SelectedRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectedRoadUiPlugin)
            .add_event::<OnSelectRoadRequested>()
            .insert_resource(SelectedRoad::default())
            .add_systems(
                Update,
                handle_select_requests
                    .in_set(GameRunningSet::HandleCommands)
                    .run_if(in_state(GameMode::RoadDrawer)),
            );
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

#[derive(Event)]
pub struct OnSelectRoadRequested {
    road_name: String,
}

impl OnSelectRoadRequested {
    pub fn new(road_name: String) -> Self {
        Self { road_name }
    }
}

fn handle_select_requests(
    mut requests: EventReader<OnSelectRoadRequested>,
    mut selected_road: ResMut<SelectedRoad>,
) {
    for request in requests.read() {
        let road_data = load_road_data(&request.road_name).expect("RoadData should load from file");

        println!("Select road: {:?}", road_data.clone());

        selected_road.set_selected_road(road_data);
    }
}
