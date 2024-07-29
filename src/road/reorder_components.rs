use bevy::prelude::*;

use crate::GameRunningSet;

use super::{OnActiveRoadModified, RoadEditor};

pub struct ReorderRoadComponentsPlugin;

impl Plugin for ReorderRoadComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnReorderRoadComponentRequested>()
            .add_systems(
                Update,
                handle_reorder_requests.in_set(GameRunningSet::HandleCommands),
            );
    }
}

#[derive(Event)]
pub struct OnReorderRoadComponentRequested {
    item_index: u8,
    requested_index: u8,
}

impl OnReorderRoadComponentRequested {
    pub fn new(item_index: u8, requested_index: u8) -> Self {
        Self {
            item_index,
            requested_index,
        }
    }
}

fn handle_reorder_requests(
    mut requests: EventReader<OnReorderRoadComponentRequested>,
    mut road_editor: ResMut<RoadEditor>,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    for request in requests.read() {
        road_editor.road.components.swap(
            request.item_index as usize,
            request.requested_index as usize,
        );

        on_road_modified.send(OnActiveRoadModified::new(road_editor.road.clone()));
    }
}
