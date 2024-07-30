use bevy::prelude::*;

use crate::{
    road::save::OnSaveActiveRoadRequested, ui::buttons::OnSaveButtonPressed, GameRunningSet,
};

// TODO: replace plugin with generic system that can be called for each button press & request event combination (or refactor to ButtonAction)

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
        println!("Save button pressed!");

        requests.send(OnSaveActiveRoadRequested);
    }
}
