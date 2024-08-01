use bevy::prelude::*;

use crate::{road::OnLoadActiveRoadRequested, ui::buttons::OnLoadButtonPressed, GameRunningSet};

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_load_requests.in_set(GameRunningSet::SendCommands),
        );
    }
}

fn send_load_requests(
    mut events: EventReader<OnLoadButtonPressed>,
    mut requests: EventWriter<OnLoadActiveRoadRequested>,
) {
    for _ in events.read() {
        let road_name = String::from("Example road");

        requests.send(OnLoadActiveRoadRequested::new(road_name));
    }
}
