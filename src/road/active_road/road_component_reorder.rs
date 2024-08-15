use bevy::prelude::*;

use crate::{ui::list::reorder_list_item::OnListItemReordered, GameRunningSet};

use super::{ActiveRoad, OnActiveRoadModified};

pub struct RoadComponentReorderPlugin;

impl Plugin for RoadComponentReorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentReorderRequested>()
            .add_event::<OnRoadComponentReordered>()
            .add_systems(
                Update,
                handle_reorder_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event)]
pub struct OnRoadComponentReorderRequested {
    component_index: usize,
    requested_index: usize,
    component_list_entity: Entity,
}

impl OnRoadComponentReorderRequested {
    pub fn new(
        component_index: usize,
        requested_index: usize,
        component_list_entity: Entity,
    ) -> Self {
        Self {
            component_index,
            requested_index,
            component_list_entity,
        }
    }
}

#[derive(Event)]
pub struct OnRoadComponentReordered {
    component_index: usize,
    previous_index: usize,
}

impl OnRoadComponentReordered {
    pub fn new(component_index: usize, previous_index: usize) -> Self {
        Self {
            component_index,
            previous_index,
        }
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }

    pub fn previous_index(&self) -> usize {
        self.previous_index
    }
}

fn handle_reorder_requests(
    mut requests: EventReader<OnRoadComponentReorderRequested>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
    mut on_component_reordered: EventWriter<OnRoadComponentReordered>,
    mut on_list_item_reordered: EventWriter<OnListItemReordered>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests.read() {
        active_road.reorder_road_components(request.component_index, request.requested_index);
        active_road.send_road_modified_event(&mut on_road_modified);

        on_component_reordered.send(OnRoadComponentReordered::new(
            request.requested_index,
            request.component_index,
        ));

        on_list_item_reordered.send(OnListItemReordered::new(request.component_list_entity));
    }
}
