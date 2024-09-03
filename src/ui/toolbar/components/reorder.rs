use bevy::prelude::*;

use crate::{
    road::active_road::active_road_events::{
        road_component_reorder::RoadComponentReorder, ActiveRoadChangeRequest,
        OnActiveRoadChangeRequested,
    },
    ui::{
        list::{
            reorder_button::{OnReorderButtonPressed, ReorderDirection},
            reorder_list::ReorderIndices,
            ListItem,
        },
        toolbar::RoadComponentsList,
    },
    GameRunningSet,
};

pub struct ReorderPlugin;

impl Plugin for ReorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_reorder_requests.in_set(GameRunningSet::SendCommands),
        );
    }
}

fn send_reorder_requests(
    mut events: EventReader<OnReorderButtonPressed>,
    mut requests: EventWriter<OnActiveRoadChangeRequested>,
    road_component_list_query: Query<&RoadComponentsList>,
    list_item_query: Query<&ListItem>,
) {
    for event in events
        .read()
        .filter(|event| road_component_list_query.contains(event.list_entity()))
    {
        let list_item = list_item_query.get(event.list_item_entity()).unwrap();

        let requested_index = match event.direction() {
            ReorderDirection::Next => list_item.index() + 1,
            ReorderDirection::Previous => list_item.index().saturating_sub(1),
        };

        requests.send(OnActiveRoadChangeRequested::new(
            ActiveRoadChangeRequest::ReorderRoadComponent(RoadComponentReorder::new(
                ReorderIndices::new(list_item.index(), requested_index),
            )),
        ));
    }
}
