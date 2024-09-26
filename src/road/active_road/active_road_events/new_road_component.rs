use bevy::prelude::*;

use crate::{
    road::{
        active_road::{changed_component_indices::ChangedComponentIndices, ActiveRoad},
        road_component::RoadComponent,
        road_data::RoadData,
    },
    ui::{list::list_events::new_list_item::OnListItemAdded, toolbar::RoadComponentsList},
    utility::changed_value::ChangedValue,
    GameRunningSet,
};

pub struct NewRoadComponentPlugin;

impl Plugin for NewRoadComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnNewRoadComponentRequested>()
            .add_event::<OnRoadComponentAdded>()
            .add_systems(
                Update,
                handle_new_component_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event, Clone, PartialEq, Debug)]
pub struct OnNewRoadComponentRequested {
    pub new_component: RoadComponent,
}

impl OnNewRoadComponentRequested {
    pub fn new(new_component: RoadComponent) -> Self {
        Self { new_component }
    }
}

#[derive(Event, Clone, PartialEq, Debug)]
pub struct OnRoadComponentAdded {
    pub new_component: RoadComponent,
    pub changed_road_data: ChangedValue<RoadData>,
    pub changed_component_indices: ChangedComponentIndices,
}

impl OnRoadComponentAdded {
    pub fn new(
        new_component: RoadComponent,
        changed_road_data: ChangedValue<RoadData>,
        changed_component_indices: ChangedComponentIndices,
    ) -> Self {
        Self {
            new_component,
            changed_road_data,
            changed_component_indices,
        }
    }

    pub fn previous_road_data(&self) -> &RoadData {
        &self.changed_road_data.previous_value
    }

    pub fn new_road_data(&self) -> &RoadData {
        &self.changed_road_data.new_value
    }
}

fn handle_new_component_requests(
    mut requests: EventReader<OnNewRoadComponentRequested>,
    mut on_added: EventWriter<OnRoadComponentAdded>,
    mut on_list_item_added: EventWriter<OnListItemAdded>,
    mut active_road: ResMut<ActiveRoad>,
    road_components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for request in requests.read() {
        let previous_road_data = active_road.road_data().clone();

        let new_component_index = active_road.component_count();
        let changed_component_indices =
            ChangedComponentIndices::new(vec![ChangedValue::new(None, Some(new_component_index))]);

        active_road.add_road_component(request.new_component.clone());
        active_road.update_road_marking_positions(&previous_road_data, &changed_component_indices);

        let new_road_data = active_road.road_data().clone();

        on_added.send(OnRoadComponentAdded::new(
            request.new_component.clone(),
            ChangedValue::new(previous_road_data, new_road_data),
            changed_component_indices,
        ));

        if let Ok(road_components_list_entity) = road_components_list_query.get_single() {
            on_list_item_added.send(OnListItemAdded::new(road_components_list_entity));
        }
    }
}
