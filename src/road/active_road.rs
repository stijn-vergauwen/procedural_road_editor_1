pub mod new_road_component;
pub mod road_component_change;
pub mod road_component_deletion;
pub mod road_component_reorder;

use bevy::{color::palettes::tailwind::*, prelude::*};
use new_road_component::NewRoadComponentPlugin;
use road_component_change::RoadComponentChangePlugin;
use road_component_deletion::RoadComponentDeletionPlugin;
use road_component_reorder::RoadComponentReorderPlugin;

use crate::ui::{toolbar::components::RoadComponentItem, ListItem};

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
        .add_event::<OnActiveRoadSet>()
        .add_event::<OnActiveRoadModified>()
        .add_systems(Startup, setup_example_road);
    }
}

fn setup_example_road(mut commands: Commands, mut on_road_set: EventWriter<OnActiveRoadSet>) {
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

    on_road_set.send(OnActiveRoadSet::new(road));
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

    pub fn component_at_index(&self, index: usize) -> &RoadComponent {
        &self.road_data().components()[index]
    }

    pub fn component_count(&self) -> usize {
        self.road_data.components().len()
    }

    pub fn set_road_data(&mut self, road: RoadData) {
        self.road_data = road;
    }

    pub fn add_road_component(&mut self, component_data: RoadComponent) -> usize {
        self.road_data.components_mut().push(component_data);

        self.component_count() - 1
    }

    pub fn reorder_road_components(
        &mut self,
        component_index: usize,
        requested_component_index: usize,
    ) {
        self.road_data
            .components_mut()
            .swap(component_index, requested_component_index);
    }

    pub fn set_road_component(&mut self, component_index: usize, component_data: RoadComponent) {
        self.road_data.components_mut()[component_index] = component_data;
    }

    pub fn delete_road_component(&mut self, component_index: usize) {
        self.road_data.components_mut().remove(component_index);
    }

    pub fn set_road_preview_entity(&mut self, road_preview_entity: Option<Entity>) {
        self.road_preview_entity = road_preview_entity;
    }

    pub fn send_road_modified_event(
        &self,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        on_road_modified.send(OnActiveRoadModified::new(self.road_data.clone()));
    }
}

#[derive(Event)]
pub struct OnActiveRoadSet {
    road_data: RoadData,
}

impl OnActiveRoadSet {
    pub fn new(road_data: RoadData) -> Self {
        Self { road_data }
    }

    pub fn road_data(&self) -> &RoadData {
        &self.road_data
    }
}

#[derive(Event, Clone)]
pub struct OnActiveRoadModified {
    road_data: RoadData,
}

impl OnActiveRoadModified {
    pub fn new(road_data: RoadData) -> Self {
        Self { road_data }
    }

    pub fn road_data(&self) -> &RoadData {
        &self.road_data
    }
}

// Thought: Having this logic here feels wrong because "it should be on the UI side of the code".
//          But I think the actual problem is that I split the UI code to it's own folder instead of keeping it close to the related functionality,
//          if I had organized it differently this logic here probably wouldn't feel out of place.
fn get_index_of_component_item(
    component_item_query: &Query<&ListItem, With<RoadComponentItem>>,
    component_item_entity: Entity,
) -> usize {
    component_item_query
        .get(component_item_entity)
        .unwrap()
        .index()
}
