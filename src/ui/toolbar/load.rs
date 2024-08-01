use bevy::prelude::*;

use crate::{
    road::OnLoadActiveRoadRequested, ui::buttons::OnLoadButtonPressed, GameRunningSet,
};

// TODO: replace plugin with generic system that can be called for each button press & request event combination (or refactor to ButtonAction)

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
        println!("Load button pressed!");

        requests.send(OnLoadActiveRoadRequested);
    }
}
