use bevy::prelude::*;

use crate::{road::RoadComponent, GameRunningSet};

use super::{ActiveRoad, OnActiveRoadModified};

pub struct RoadComponentChangePlugin;

impl Plugin for RoadComponentChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentChangeRequested>()
            .add_systems(
                Update,
                handle_change_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event)]
pub struct OnRoadComponentChangeRequested {
    component_index: usize,
    #[allow(unused)]
    current_data: RoadComponent,
    requested_data: RoadComponent,
}

impl OnRoadComponentChangeRequested {
    pub fn new(
        component_index: usize,
        current_data: RoadComponent,
        requested_data: RoadComponent,
    ) -> Self {
        Self {
            component_index,
            current_data,
            requested_data,
        }
    }
}

fn handle_change_requests(
    mut requests: EventReader<OnRoadComponentChangeRequested>,
    mut active_road: ResMut<ActiveRoad>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    for request in requests.read() {
        active_road.set_road_component(
            request.component_index,
            request.requested_data.clone(),
            &mut on_road_modified,
        );
    }
}
