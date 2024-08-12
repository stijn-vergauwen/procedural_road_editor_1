use bevy::prelude::*;

use crate::{
    ui::{toolbar::components::RoadComponentItem, ListItem},
    GameRunningSet,
};

use super::{get_index_of_component_item, ActiveRoad, OnActiveRoadModified};

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
}

impl OnRoadComponentDeletionRequested {
    pub fn new(component_entity: Entity) -> Self {
        Self { component_entity }
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
    component_item_query: Query<&ListItem, With<RoadComponentItem>>,
) {
    for request in requests.read() {
        let component_index =
            get_index_of_component_item(&component_item_query, request.component_entity);

        active_road.delete_road_component(component_index);
        active_road.send_road_modified_event(&mut on_road_modified);

        on_deleted.send(OnRoadComponentDeleted::new(
            request.component_entity,
            component_index,
        ));
    }
}
