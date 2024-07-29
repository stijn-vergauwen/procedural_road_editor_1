use std::{fs, io::Error};

use bevy::prelude::*;

use crate::GameRunningSet;

use super::{RoadData, RoadEditor};

// TODO: add save button
// TODO: save road on save button click

pub struct SaveRoadPlugin;

impl Plugin for SaveRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnActiveRoadSaveRequested>().add_systems(
            Update,
            handle_save_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Event)]
pub struct OnActiveRoadSaveRequested;

fn handle_save_requests(
    mut requests: EventReader<OnActiveRoadSaveRequested>,
    road_editor: Res<RoadEditor>,
) {
    for _ in requests.read() {
        let file_name = road_editor.road().name.as_str();
        let Ok(road_data) = serialize_road_data(road_editor.road()) else {
            warn!("Road data did not serialize successfully!");
            return;
        };

        if let Err(_) = save_data_to_asset_folder(road_data, file_name) {
            warn!("Road data did not save successfully!");
        }
    }
}

fn serialize_road_data(road_data: &RoadData) -> Result<String, ron::Error> {
    ron::to_string(road_data)
}

fn save_data_to_asset_folder(data: String, file_name: &str) -> Result<(), Error> {
    fs::write(format!("assets/roads/{file_name}"), data)
}

// fn deserialize_road_data(serialized_data: &str) -> Result<RoadData, ron::Error> {
//     Ok(ron::from_str::<RoadData>(serialized_data)?)
// }
