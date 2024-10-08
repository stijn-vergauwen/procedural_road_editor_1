pub mod active_road_events;
pub mod changed_component_indices;

use active_road_events::{
    road_component_change::RoadComponentFieldChange, ActiveRoadEventsPlugin, OnActiveRoadSet,
};
use bevy::{color::palettes::tailwind::*, prelude::*};
use changed_component_indices::ChangedComponentIndices;

use crate::{game_modes::GameMode, ui::list::list_events::list_reorder::ReorderIndices};

use super::{road_component::RoadComponent, road_data::RoadData, road_marking::RoadMarking};

pub struct ActiveRoadPlugin;

impl Plugin for ActiveRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ActiveRoadEventsPlugin,))
            .add_systems(OnEnter(GameMode::RoadEditor), setup_example_road)
            .add_systems(OnExit(GameMode::RoadEditor), despawn_active_road);
    }
}

fn setup_example_road(mut commands: Commands, mut on_road_set: EventWriter<OnActiveRoadSet>) {
    let road_components = vec![
        RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.3), GRAY_600),
        RoadComponent::new("Lane", Vec2::new(4.0, 0.1), GRAY_800),
        RoadComponent::new("Lane", Vec2::new(4.0, 0.1), GRAY_800),
        RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.3), GRAY_600),
    ];

    let road_markings = vec![
        RoadMarking::new(-3.7, 0.15, Color::WHITE),
        RoadMarking::new(3.7, 0.15, Color::WHITE),
    ];

    let road = RoadData::new(String::from("Example road"), road_components, road_markings);

    let active_road = ActiveRoad {
        road_data: road.clone(),
        road_preview_entity: None,
    };

    commands.insert_resource(active_road);

    on_road_set.send(OnActiveRoadSet::new(road));
}

fn despawn_active_road(mut commands: Commands, active_road: Res<ActiveRoad>) {
    if let Some(road_preview_entity) = active_road.road_preview_entity {
        commands.entity(road_preview_entity).despawn_recursive();
    }

    commands.remove_resource::<ActiveRoad>();
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

    pub fn add_road_component(&mut self, road_component: RoadComponent) {
        self.road_data.components_mut().push(road_component);
        self.road_data.recalculate_road_component_positions();
    }

    pub fn reorder_road_components(&mut self, reorder: ReorderIndices) {
        self.road_data
            .components_mut()
            .swap(reorder.previous_index, reorder.new_index);
        self.road_data.recalculate_road_component_positions();
    }

    pub fn set_road_component(&mut self, component_index: usize, component_data: RoadComponent) {
        self.road_data.components_mut()[component_index] = component_data;
        self.road_data.recalculate_road_component_positions();
    }

    pub fn change_road_component_at_index(
        &mut self,
        component_index: usize,
        field_to_change: RoadComponentFieldChange,
    ) {
        let road_component = self.component_at_index(component_index).clone();

        let new_component = match field_to_change {
            RoadComponentFieldChange::Name(name) => road_component.with_name(name),
            RoadComponentFieldChange::Width(width) => road_component.with_width(width),
            RoadComponentFieldChange::Height(height) => road_component.with_height(height),
            RoadComponentFieldChange::Color(color) => road_component.with_color(color),
        };

        self.set_road_component(component_index, new_component.clone());
    }

    pub fn delete_road_component(&mut self, component_index: usize) {
        self.road_data.components_mut().remove(component_index);
        self.road_data.recalculate_road_component_positions();
    }

    pub fn set_road_preview_entity(&mut self, road_preview_entity: Option<Entity>) {
        self.road_preview_entity = road_preview_entity;
    }

    /// Updates each road marking to keep them in the same spot relative to the road component they're on.
    pub fn update_road_marking_positions(
        &mut self,
        previous_road_data: &RoadData,
        changed_component_indices: &ChangedComponentIndices,
    ) {
        let delta_component_positions = changed_component_indices
            .calculate_delta_component_positions(
                previous_road_data.component_positions(),
                self.road_data.component_positions(),
            );

        for road_marking in self.road_data.markings_mut() {
            let Some(road_component_under_point) =
                previous_road_data.find_road_component_under_point(road_marking.x_position)
            else {
                continue;
            };

            let Some(Some(delta_position)) =
                delta_component_positions.get(road_component_under_point.road_component_index)
            else {
                continue;
            };

            road_marking.x_position +=
                delta_position.get_field(road_component_under_point.closest_position_field);
        }
    }
}
