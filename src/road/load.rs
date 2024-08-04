use std::{fs, io};

use bevy::prelude::*;

use crate::GameRunningSet;

use super::{OnActiveRoadModified, ActiveRoad, RoadData};

// TODO: see if I can use the Bevy asset systems for saving / loading (instead of fs)

pub struct LoadRoadPlugin;

impl Plugin for LoadRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnLoadRoadRequested>().add_systems(
            Update,
            handle_load_requests.in_set(GameRunningSet::HandleCommands),
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
    mut active_road: ResMut<ActiveRoad>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    for request in requests.read() {
        let Ok(serialized_data) = load_data_from_asset_folder(request.road_name()) else {
            warn!("Something went wrong while loading road data!");
            return;
        };

        let Ok(road_data) = deserialize_road_data(&serialized_data) else {
            warn!("Something went wrong while deserializing road data!");
            return;
        };

        active_road.set_road_data(road_data, &mut on_road_modified);
    }
}

fn deserialize_road_data(serialized_data: &str) -> Result<RoadData, ron::Error> {
    Ok(ron::from_str::<RoadData>(serialized_data)?)
}

fn load_data_from_asset_folder(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(format!("assets/roads/{file_name}.ron"))
}
