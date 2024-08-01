mod active_road;

pub use active_road::{ActiveRoad, OnActiveRoadModified};
use bevy::prelude::*;

use super::{road_data::RoadData, RoadComponent};

pub struct RoadEditorPlugin;

impl Plugin for RoadEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnActiveRoadModified>()
            .add_systems(Startup, setup_example_road);
    }
}

fn setup_example_road(
    mut commands: Commands,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    let road_components = vec![
        RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.2)),
        RoadComponent::new("Lane", Vec2::new(4.0, 0.05)),
        RoadComponent::new("Median", Vec2::new(0.4, 0.2)),
        RoadComponent::new("Lane", Vec2::new(4.0, 0.05)),
        RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.2)),
    ];

    let road = RoadData::new(String::from("Example road"), road_components);

    let road_editor = RoadEditor { road: road.clone() };

    commands.insert_resource(road_editor);

    on_road_modified.send(OnActiveRoadModified::new(road));
}

#[derive(Resource)]
pub struct RoadEditor {
    road: RoadData,
}

impl RoadEditor {
    pub fn road(&self) -> &RoadData {
        &self.road
    }

    pub fn set_road(
        &mut self,
        road: RoadData,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road = road.clone();

        on_road_modified.send(OnActiveRoadModified::new(road));
    }

    pub fn reorder_road_components(
        &mut self,
        component_index: usize,
        requested_component_index: usize,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road
            .components_mut()
            .swap(component_index, requested_component_index);

        on_road_modified.send(OnActiveRoadModified::new(self.road.clone()));
    }
}
