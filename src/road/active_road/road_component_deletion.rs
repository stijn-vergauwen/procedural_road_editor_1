use bevy::prelude::*;

use crate::GameRunningSet;

use super::{ActiveRoad, OnActiveRoadModified};

pub struct RoadComponentDeletionPlugin;

impl Plugin for RoadComponentDeletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentDeletionRequested>()
            .add_event::<OnRoadComponentDeleted>()
            .add_systems(
                Update,
                handle_deletion_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event)]
pub struct OnRoadComponentDeletionRequested {
    component_entity: Entity,
    component_index: usize,
}

impl OnRoadComponentDeletionRequested {
    pub fn new(component_entity: Entity, component_index: usize) -> Self {
        Self {
            component_entity,
            component_index,
        }
    }
}

#[derive(Event)]
pub struct OnRoadComponentDeleted {
    component_entity: Entity,
    component_index: usize,
}

impl OnRoadComponentDeleted {
    pub fn new(component_entity: Entity, component_index: usize) -> Self {
        Self {
            component_entity,
            component_index,
        }
    }

    pub fn component_entity(&self) -> Entity {
        self.component_entity
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }
}

fn handle_deletion_requests(
    mut requests: EventReader<OnRoadComponentDeletionRequested>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
    mut on_deleted: EventWriter<OnRoadComponentDeleted>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests.read() {
        active_road.delete_road_component(request.component_index);
        active_road.send_road_modified_event(&mut on_road_modified);

        on_deleted.send(OnRoadComponentDeleted::new(
            request.component_entity,
            request.component_index,
        ));
    }
}
