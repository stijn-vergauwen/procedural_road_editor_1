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
        RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.3)),
        RoadComponent::new("Lane", Vec2::new(4.0, 0.1)),
        RoadComponent::new("Median", Vec2::new(0.4, 0.3)),
        RoadComponent::new("Lane", Vec2::new(4.0, 0.1)),
        RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.3)),
    ];

    let road = RoadData::new(String::from("Example road"), road_components);

    let active_road = ActiveRoad {
        road_data: road.clone(),
        road_preview_entity: None,
    };

    commands.insert_resource(active_road);

    on_road_modified.send(OnActiveRoadModified::new(road, None));
}

#[derive(Resource)]
pub struct ActiveRoad {
    road_data: RoadData,
    road_preview_entity: Option<Entity>,
}

impl ActiveRoad {
    pub fn road_data(&self) -> &RoadData {
        &self.road_data
    }

    pub fn set_road_data(
        &mut self,
        road: RoadData,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road_data = road.clone();

        on_road_modified.send(OnActiveRoadModified::new(road, self.road_preview_entity));
    }

    pub fn reorder_road_components(
        &mut self,
        component_index: usize,
        requested_component_index: usize,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road_data
            .components_mut()
            .swap(component_index, requested_component_index);

        on_road_modified.send(OnActiveRoadModified::new(
            self.road_data.clone(),
            self.road_preview_entity,
        ));
    }

    pub fn set_road_preview_entity(&mut self, road_preview_entity: Option<Entity>) {
        self.road_preview_entity = road_preview_entity;
    }
}

#[derive(Event, Clone)]
pub struct OnActiveRoadModified {
    road_data: RoadData,
    road_preview_entity: Option<Entity>,
}

impl OnActiveRoadModified {
    pub fn new(road_data: RoadData, road_preview_entity: Option<Entity>) -> Self {
        Self {
            road_data,
            road_preview_entity,
        }
    }

    pub fn road_data(&self) -> &RoadData {
        &self.road_data
    }

    pub fn road_preview_entity(&self) -> Option<Entity> {
        self.road_preview_entity
    }
}
