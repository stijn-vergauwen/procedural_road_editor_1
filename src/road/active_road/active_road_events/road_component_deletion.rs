use bevy::prelude::*;

use crate::{
    road::ActiveRoad,
    ui::{
        list::delete_list_item::{ListItemDeletion, OnListItemDeletionRequested},
        toolbar::RoadComponentsList,
    },
    GameRunningSet,
};

use super::{ActiveRoadChange, OnActiveRoadChangeRequested, OnActiveRoadChanged, RoadDataChange};

pub struct RoadComponentDeletionPlugin;

impl Plugin for RoadComponentDeletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_component_deletion_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct RoadComponentDeletion {
    pub index_to_delete: usize,
}

impl RoadComponentDeletion {
    pub fn new(index_to_delete: usize) -> Self {
        Self { index_to_delete }
    }
}

fn handle_component_deletion_requests(
    mut requests: EventReader<OnActiveRoadChangeRequested>,
    mut on_changed: EventWriter<OnActiveRoadChanged>,
    mut on_list_item_deleted: EventWriter<OnListItemDeletionRequested>,
    mut active_road: ResMut<ActiveRoad>,
    road_components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for request in requests.read() {
        let deletion_request = match &request.change_request {
            ActiveRoadChange::DeleteRoadComponent(request) => request,
            _ => continue,
        };

        let previous_road_data = active_road.road_data().clone();

        active_road.delete_road_component(deletion_request.index_to_delete);

        let new_road_data = active_road.road_data().clone();

        on_changed.send(OnActiveRoadChanged::new(
            request.change_request.clone(),
            RoadDataChange::new(previous_road_data, new_road_data),
        ));

        if let Ok(road_components_list_entity) = road_components_list_query.get_single() {
            on_list_item_deleted.send(OnListItemDeletionRequested::new(ListItemDeletion::new(
                road_components_list_entity,
                deletion_request.index_to_delete,
            )));
        }
    }
}
