use bevy::prelude::*;

use crate::{
    road::{OnSaveRoadRequested, RoadEditor},
    ui::buttons::OnSaveButtonPressed,
    GameRunningSet,
};

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
    mut requests: EventWriter<OnSaveRoadRequested>,
    road_editor: Res<RoadEditor>,
) {
    for _ in events.read() {
        requests.send(OnSaveRoadRequested::new(road_editor.road().clone()));
    }
}
