use std::{fs, io::Error};

use bevy::prelude::*;
use ron::ser::PrettyConfig;

use crate::GameRunningSet;

use super::{RoadData, RoadEditor};

// TODO: see if I can use the Bevy asset systems for saving / loading (instead of fs)

pub struct SaveRoadPlugin;

impl Plugin for SaveRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSaveActiveRoadRequested>().add_systems(
            Update,
            handle_save_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Event)]
pub struct OnSaveActiveRoadRequested;

fn handle_save_requests(
    mut requests: EventReader<OnSaveActiveRoadRequested>,
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
    ron::ser::to_string_pretty(road_data, PrettyConfig::new().struct_names(true))
}

fn save_data_to_asset_folder(data: String, file_name: &str) -> Result<(), Error> {
    fs::write(format!("assets/roads/{file_name}.ron"), data)
}
