use std::{fs, io};

use bevy::prelude::*;

use crate::GameRunningSet;

use super::{OnActiveRoadModified, RoadData, RoadEditor};

// TODO: see if I can use the Bevy asset systems for saving / loading (instead of fs)

pub struct LoadRoadPlugin;

impl Plugin for LoadRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnLoadActiveRoadRequested>().add_systems(
            Update,
            handle_load_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Event)]
pub struct OnLoadActiveRoadRequested;

fn handle_load_requests(
    mut requests: EventReader<OnLoadActiveRoadRequested>,
    mut road_editor: ResMut<RoadEditor>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    for _ in requests.read() {
        // TODO: get file_name from request
        let file_name = "Example road";
        let Ok(serialized_data) = load_data_from_asset_folder(file_name) else {
            warn!("Road data did not load successfully!");
            return;
        };

        let Ok(road_data) = deserialize_road_data(&serialized_data) else {
            warn!("Road data did not save successfully!");
            return;
        };

        road_editor.set_road(road_data, &mut on_road_modified);
    }
}

fn deserialize_road_data(serialized_data: &str) -> Result<RoadData, ron::Error> {
    Ok(ron::from_str::<RoadData>(serialized_data)?)
}

fn load_data_from_asset_folder(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(format!("assets/roads/{file_name}.ron"))
}
