use std::{fs, io};

use ron::ser::PrettyConfig;

use super::road_data::RoadData;

// Saving

pub fn save_road_data(road_data: &RoadData, file_name: &str) -> Result<(), ron::Error> {
    Ok(save_data_to_asset_folder(
        serialize_road_data(road_data)?,
        file_name,
    )?)
}

fn serialize_road_data(road_data: &RoadData) -> Result<String, ron::Error> {
    ron::ser::to_string_pretty(road_data, PrettyConfig::new().struct_names(true))
}

fn save_data_to_asset_folder(data: String, file_name: &str) -> Result<(), io::Error> {
    fs::write(path_from_file_name(file_name), data)
}

// Loading

pub fn load_road_data(file_name: &str) -> Result<RoadData, ron::Error> {
    deserialize_road_data(&load_data_from_asset_folder(file_name)?)
}

fn deserialize_road_data(serialized_data: &str) -> Result<RoadData, ron::Error> {
    Ok(ron::from_str::<RoadData>(serialized_data)?)
}

fn load_data_from_asset_folder(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(path_from_file_name(file_name))
}

// Utility

fn path_from_file_name(file_name: &str) -> String {
    format!("assets/roads/{file_name}.ron")
}
