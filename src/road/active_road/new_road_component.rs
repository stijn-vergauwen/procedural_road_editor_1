use bevy::prelude::*;

use crate::{road::RoadComponent, ui::list::add_list_item::OnListItemAdded, GameRunningSet};

use super::{ActiveRoad, OnActiveRoadModified};

pub struct NewRoadComponentPlugin;

impl Plugin for NewRoadComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnNewRoadComponentRequested>()
            .add_event::<OnRoadComponentAdded>()
            .add_systems(
                Update,
                handle_change_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event)]
pub struct OnNewRoadComponentRequested {
    component_data: RoadComponent,
    component_list_entity: Entity,
}

impl OnNewRoadComponentRequested {
    pub fn new(component_data: RoadComponent, component_list_entity: Entity) -> Self {
        Self {
            component_data,
            component_list_entity,
        }
    }
}

#[derive(Event)]
pub struct OnRoadComponentAdded {
    component_data: RoadComponent,
    // TODO: check if field is needed
    component_index: usize,
    // TODO: check if field is needed
    component_count: usize,
}

impl OnRoadComponentAdded {
    pub fn new(
        component_data: RoadComponent,
        component_index: usize,
        component_count: usize,
    ) -> Self {
        Self {
            component_data,
            component_index,
            component_count,
        }
    }

    pub fn component_data(&self) -> &RoadComponent {
        &self.component_data
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }

    pub fn component_count(&self) -> usize {
        self.component_count
    }
}

fn handle_change_requests(
    mut requests: EventReader<OnNewRoadComponentRequested>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
    mut on_component_added: EventWriter<OnRoadComponentAdded>,
    mut on_list_item_added: EventWriter<OnListItemAdded>,
    mut active_road: ResMut<ActiveRoad>,
) {
    for request in requests.read() {
        let component_index = active_road.add_road_component(request.component_data.clone());
        active_road.send_road_modified_event(&mut on_road_modified);

        on_component_added.send(OnRoadComponentAdded::new(
            request.component_data.clone(),
            component_index,
            active_road.component_count(),
        ));

        on_list_item_added.send(OnListItemAdded::new(request.component_list_entity));
    }
}
