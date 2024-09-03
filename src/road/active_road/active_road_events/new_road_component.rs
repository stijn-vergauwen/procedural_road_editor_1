use bevy::prelude::*;

use crate::{
    road::{ActiveRoad, RoadComponent},
    ui::{list::add_list_item::OnListItemAdded, toolbar::RoadComponentsList},
    GameRunningSet,
};

use super::{ActiveRoadChange, OnActiveRoadChangeRequested, OnActiveRoadChanged, RoadDataChange};

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
pub struct NewRoadComponent {
    pub road_component: RoadComponent,
}

impl NewRoadComponent {
    pub fn new(road_component: RoadComponent) -> Self {
        Self { road_component }
    }
}

fn handle_new_component_requests(
    mut requests: EventReader<OnActiveRoadChangeRequested>,
    mut on_changed: EventWriter<OnActiveRoadChanged>,
    mut on_list_item_added: EventWriter<OnListItemAdded>,
    mut active_road: ResMut<ActiveRoad>,
    road_components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for request in requests.read() {
        let add_request = match &request.change_request {
            ActiveRoadChange::AddRoadComponent(request) => request,
            _ => continue,
        };

        let new_road_component = add_request.road_component.clone();
        let previous_road_data = active_road.road_data().clone();

        active_road.add_road_component(new_road_component.clone());

        let new_road_data = active_road.road_data().clone();

        on_changed.send(OnActiveRoadChanged::new(
            request.change_request.clone(),
            RoadDataChange::new(previous_road_data, new_road_data),
        ));

        if let Ok(road_components_list_entity) = road_components_list_query.get_single() {
            on_list_item_added.send(OnListItemAdded::new(road_components_list_entity));
        }
    }
}
