use bevy::prelude::*;

use crate::{road::RoadComponent, GameRunningSet};

use super::{ActiveRoad, OnActiveRoadModified};

pub struct RoadComponentChangePlugin;

impl Plugin for RoadComponentChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentChangeRequested>()
            .add_event::<OnRoadComponentChanged>()
            .add_systems(
                Update,
                handle_change_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event)]
pub struct OnRoadComponentChangeRequested {
    requested_change: Box<dyn Fn(RoadComponent) -> RoadComponent + Send + Sync>,
    component_entity: Entity,
    component_index: usize,
}

impl OnRoadComponentChangeRequested {
    pub fn new(
        requested_change: Box<dyn Fn(RoadComponent) -> RoadComponent + Send + Sync>,
        component_entity: Entity,
        component_index: usize,
    ) -> Self {
        Self {
            requested_change: Box::new(requested_change),
            component_entity,
            component_index,
        }
    }
}

#[derive(Event)]
pub struct OnRoadComponentChanged {
    component_data: RoadComponent,
    component_entity: Entity,
    // TODO: check if index is needed
    component_index: usize,
}

impl OnRoadComponentChanged {
    pub fn new(
        component_data: RoadComponent,
        component_entity: Entity,
        component_index: usize,
    ) -> Self {
        Self {
            component_data,
            component_entity,
            component_index,
        }
    }

    pub fn component_data(&self) -> &RoadComponent {
        &self.component_data
    }

    pub fn component_entity(&self) -> Entity {
        self.component_entity
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }
}

fn handle_change_requests(
    mut requests: EventReader<OnRoadComponentChangeRequested>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
    mut on_changed: EventWriter<OnRoadComponentChanged>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests.read() {
        let current_data = active_road.component_at_index(request.component_index);
        let changed_data = (request.requested_change)(current_data.clone());

        active_road.set_road_component(request.component_index, changed_data.clone());
        active_road.send_road_modified_event(&mut on_road_modified);

        on_changed.send(OnRoadComponentChanged::new(
            changed_data,
            request.component_entity,
            request.component_index,
        ));
    }
}
