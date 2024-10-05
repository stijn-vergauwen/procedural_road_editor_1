use bevy::prelude::*;

use crate::GameRunningSet;

use crate::road::road_data::RoadData;

use super::persistance::save_road_data;

// TODO: see if I can use the Bevy asset systems for saving / loading (instead of fs)

pub struct SaveRoadPlugin;

impl Plugin for SaveRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSaveRoadRequested>().add_systems(
            Update,
            handle_save_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Event)]
pub struct OnSaveRoadRequested {
    road: RoadData,
}

impl OnSaveRoadRequested {
    pub fn new(road: RoadData) -> Self {
        Self { road }
    }

    pub fn road(&self) -> &RoadData {
        &self.road
    }
}

fn handle_save_requests(mut requests: EventReader<OnSaveRoadRequested>) {
    for request in requests.read() {
        let file_name = request.road().name();

        save_road_data(request.road(), file_name).expect("RoadData should save to file.");
    }
}
