use bevy::prelude::*;

use crate::{road::OnSaveActiveRoadRequested, ui::buttons::OnSaveButtonPressed, GameRunningSet};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_save_requests.in_set(GameRunningSet::SendCommands),
        );
    }
}

fn send_save_requests(
    mut events: EventReader<OnSaveButtonPressed>,
    mut requests: EventWriter<OnSaveActiveRoadRequested>,
) {
    for _ in events.read() {
        requests.send(OnSaveActiveRoadRequested);
    }
}
