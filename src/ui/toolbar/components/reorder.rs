use bevy::prelude::*;

use crate::{
    road::reorder_components::OnReorderRoadComponentRequested,
    ui::{
        buttons::{OnReorderButtonPressed, ReorderDirection},
        toolbar::ToolBar,
        ListItem,
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
    mut requests: EventWriter<OnReorderRoadComponentRequested>,
    toolbar_query: Query<&ToolBar>,
    list_item_query: Query<&ListItem>,
) {
    for event in events
        .read()
        .filter(|event| toolbar_query.contains(event.list_entity()))
    {
        let list_item = list_item_query.get(event.list_item_entity()).unwrap();

        let requested_index = match event.direction() {
            ReorderDirection::Next => list_item.order + 1,
            ReorderDirection::Previous => list_item.order.saturating_sub(1),
        };

        requests.send(OnReorderRoadComponentRequested::new(
            list_item.order,
            requested_index,
        ));
    }
}
