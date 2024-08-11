use bevy::prelude::*;

use crate::GameRunningSet;

use super::{ActiveRoad, OnActiveRoadModified};

pub struct RoadComponentReorderPlugin;

impl Plugin for RoadComponentReorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentReorderRequested>()
            .add_systems(
                Update,
                handle_reorder_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event)]
pub struct OnRoadComponentReorderRequested {
    item_index: usize,
    requested_index: usize,
}

impl OnRoadComponentReorderRequested {
    pub fn new(item_index: usize, requested_index: usize) -> Self {
        Self {
            item_index,
            requested_index,
        }
    }
}

// TODO: add onReordered event

fn handle_reorder_requests(
    mut requests: EventReader<OnRoadComponentReorderRequested>,
    mut active_road: ResMut<ActiveRoad>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    for request in requests.read() {
        active_road.reorder_road_components(request.item_index, request.requested_index);
        active_road.send_road_modified_event(&mut on_road_modified);
    }
}
