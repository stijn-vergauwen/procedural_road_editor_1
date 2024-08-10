pub mod new_road_component;
pub mod road_component_change;
pub mod road_component_deletion;
pub mod road_component_reorder;

use bevy::{color::palettes::tailwind::*, prelude::*};
use new_road_component::NewRoadComponentPlugin;
use road_component_change::RoadComponentChangePlugin;
use road_component_deletion::RoadComponentDeletionPlugin;
use road_component_reorder::RoadComponentReorderPlugin;

use super::{road_data::RoadData, RoadComponent};

pub struct ActiveRoadPlugin;

impl Plugin for ActiveRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RoadComponentReorderPlugin,
            RoadComponentChangePlugin,
            RoadComponentDeletionPlugin,
            NewRoadComponentPlugin,
        ))
        .add_event::<OnActiveRoadModified>()
        .add_systems(Startup, setup_example_road);
    }
}

fn setup_example_road(
    mut commands: Commands,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    let road_components = vec![
        RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.3), GRAY_600.into()),
        RoadComponent::new("Lane", Vec2::new(4.0, 0.1), GRAY_800.into()),
        RoadComponent::new("Median", Vec2::new(0.4, 0.3), GRAY_600.into()),
        RoadComponent::new("Lane", Vec2::new(4.0, 0.1), GRAY_800.into()),
        RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.3), GRAY_600.into()),
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

        self.send_road_modified_event(on_road_modified);
    }

    pub fn add_road_component(
        &mut self,
        component_data: RoadComponent,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road_data.components_mut().push(component_data);

        self.send_road_modified_event(on_road_modified);
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

        self.send_road_modified_event(on_road_modified);
    }

    pub fn set_road_component(
        &mut self,
        component_index: usize,
        component_data: RoadComponent,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road_data.components_mut()[component_index] = component_data;

        self.send_road_modified_event(on_road_modified);
    }

    pub fn delete_road_component(
        &mut self,
        component_index: usize,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road_data.components_mut().remove(component_index);

        self.send_road_modified_event(on_road_modified);
    }

    pub fn set_road_preview_entity(&mut self, road_preview_entity: Option<Entity>) {
        self.road_preview_entity = road_preview_entity;
    }

    fn send_road_modified_event(&self, on_road_modified: &mut EventWriter<OnActiveRoadModified>) {
        on_road_modified.send(OnActiveRoadModified::new(
            self.road_data.clone(),
            self.road_preview_entity,
        ));
    }
}

// TODO: add road components index map to event (vec of arrays with 2 ints, one for prev index and one for new index of each component)
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
