use bevy::prelude::*;

use crate::{
    road::{ActiveRoad, RoadComponent},
    ui::{list::add_list_item::OnListItemAdded, toolbar::RoadComponentsList},
    GameRunningSet,
};

use super::{
    ActiveRoadChange, ActiveRoadChangeRequest, OnActiveRoadChangeRequested, OnActiveRoadChanged,
};

pub struct NewRoadComponentPlugin;

impl Plugin for NewRoadComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_new_component_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct NewRoadComponentRequest {
    road_component: RoadComponent,
}

impl NewRoadComponentRequest {
    pub fn new(road_component: RoadComponent) -> Self {
        Self { road_component }
    }
}

#[derive(Clone, PartialEq)]
pub struct NewRoadComponent {
    pub road_component: RoadComponent,
    pub road_component_index: usize,
}

impl NewRoadComponent {
    pub fn new(road_component: RoadComponent, road_component_index: usize) -> Self {
        Self {
            road_component,
            road_component_index,
        }
    }
}

fn handle_new_component_requests(
    mut requests: EventReader<OnActiveRoadChangeRequested>,
    mut on_changed: EventWriter<OnActiveRoadChanged>,
    mut on_list_item_added: EventWriter<OnListItemAdded>,
    mut active_road: ResMut<ActiveRoad>,
    road_components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for request in
        requests
            .read()
            .filter_map(|request| match &request.active_road_change_request {
                ActiveRoadChangeRequest::AddRoadComponent(request) => {
                    Some(request)
                }
                _ => None,
            })
    {
        let new_road_component = request.road_component.clone();
        let previous_road_data = active_road.road_data().clone();

        let road_component_index = active_road.add_road_component(new_road_component.clone());

        let new_road_data = active_road.road_data().clone();

        on_changed.send(OnActiveRoadChanged::new(
            ActiveRoadChange::RoadComponentAdded(NewRoadComponent::new(
                new_road_component,
                road_component_index,
            )),
            previous_road_data,
            new_road_data,
        ));

        if let Ok(road_components_list_entity) = road_components_list_query.get_single() {
            on_list_item_added.send(OnListItemAdded::new(road_components_list_entity));
        }
    }
}
