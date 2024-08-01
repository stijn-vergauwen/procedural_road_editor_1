use std::{fs, io::Error};

use bevy::prelude::*;
use ron::ser::PrettyConfig;

use crate::GameRunningSet;

use super::RoadData;

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
        let Ok(road_data) = serialize_road_data(request.road()) else {
            warn!("Something went wrong while serializing road data!");
            return;
        };

        if let Err(_) = save_data_to_asset_folder(road_data, file_name) {
            warn!("Something went wrong while saving road data!");
        }
    }
}

fn serialize_road_data(road_data: &RoadData) -> Result<String, ron::Error> {
    ron::ser::to_string_pretty(road_data, PrettyConfig::new().struct_names(true))
}

fn save_data_to_asset_folder(data: String, file_name: &str) -> Result<(), Error> {
    fs::write(format!("assets/roads/{file_name}.ron"), data)
}
