use bevy::prelude::*;

use crate::{game_modes::GameMode, GameRunningSet};

use super::{
    active_road::{active_road_events::OnActiveRoadSet, ActiveRoad},
    persistance::load_road_data,
};

// TODO: see if I can use the Bevy asset systems for saving / loading (instead of fs)

pub struct LoadRoadPlugin;

impl Plugin for LoadRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnLoadRoadRequested>().add_systems(
            Update,
            handle_load_requests
                .in_set(GameRunningSet::HandleCommands)
                .run_if(in_state(GameMode::RoadEditor)),
        );
    }
}

#[derive(Event)]
pub struct OnLoadRoadRequested {
    road_name: String,
}

impl OnLoadRoadRequested {
    pub fn new(road_name: String) -> Self {
        Self { road_name }
    }

    pub fn road_name(&self) -> &str {
        &self.road_name
    }
}

fn handle_load_requests(
    mut requests: EventReader<OnLoadRoadRequested>,
    mut on_road_set: EventWriter<OnActiveRoadSet>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests.read() {
        let road_data =
            load_road_data(request.road_name()).expect("RoadData should load from file");

        active_road.set_road_data(road_data.clone());

        on_road_set.send(OnActiveRoadSet::new(road_data));
    }
}
