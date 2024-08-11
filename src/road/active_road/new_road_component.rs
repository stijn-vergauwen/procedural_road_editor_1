use bevy::prelude::*;

use crate::{road::RoadComponent, GameRunningSet};

use super::{ActiveRoad, OnActiveRoadModified};

pub struct NewRoadComponentPlugin;

impl Plugin for NewRoadComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnNewRoadComponentRequested>().add_systems(
            Update,
            handle_change_requests.in_set(GameRunningSet::HandleCommands),
        );
    }
}

#[derive(Event)]
pub struct OnNewRoadComponentRequested {
    component_data: RoadComponent,
}

impl OnNewRoadComponentRequested {
    pub fn new(component_data: RoadComponent) -> Self {
        Self { component_data }
    }
}

// TODO: add onAdded event

fn handle_change_requests(
    mut requests: EventReader<OnNewRoadComponentRequested>,
    mut active_road: ResMut<ActiveRoad>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    for request in requests.read() {
        active_road.add_road_component(request.component_data.clone());
        active_road.send_road_modified_event(&mut on_road_modified);
    }
}
