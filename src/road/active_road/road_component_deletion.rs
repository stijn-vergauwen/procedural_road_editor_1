use bevy::prelude::*;

use crate::{ui::list::delete_list_item::OnListItemDeleted, GameRunningSet};

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
    list_entity: Entity,
    component_entity: Entity,
    index: usize,
}

impl OnRoadComponentDeletionRequested {
    pub fn new(list_entity: Entity, component_entity: Entity, index: usize) -> Self {
        Self {
            list_entity,
            component_entity,
            index,
        }
    }
}

fn handle_deletion_requests(
    mut requests: EventReader<OnRoadComponentDeletionRequested>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
    mut on_deleted: EventWriter<OnListItemDeleted>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests.read() {
        active_road.delete_road_component(request.index);
        active_road.send_road_modified_event(&mut on_road_modified);

        on_deleted.send(OnListItemDeleted::new(
            request.list_entity,
            request.component_entity,
            request.index,
        ));
    }
}
