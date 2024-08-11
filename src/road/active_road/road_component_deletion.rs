use bevy::prelude::*;

use crate::GameRunningSet;

use super::{ActiveRoad, OnActiveRoadModified};

pub struct RoadComponentDeletionPlugin;

impl Plugin for RoadComponentDeletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentDeletionRequested>()
            .add_systems(
                Update,
                handle_deletion_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event)]
pub struct OnRoadComponentDeletionRequested {
    component_index: usize,
}

impl OnRoadComponentDeletionRequested {
    pub fn new(component_index: usize) -> Self {
        Self { component_index }
    }
}

// TODO: add OnDeleted event

fn handle_deletion_requests(
    mut requests: EventReader<OnRoadComponentDeletionRequested>,
    mut active_road: ResMut<ActiveRoad>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    for request in requests.read() {
        active_road.delete_road_component(request.component_index);
        active_road.send_road_modified_event(&mut on_road_modified);
    }
}
