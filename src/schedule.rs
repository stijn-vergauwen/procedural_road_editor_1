use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameRunningSet::FetchData,
                GameRunningSet::GetUserInput,
                GameRunningSet::SendEvents,
                GameRunningSet::SendCommands,
                GameRunningSet::HandleCommands,
                GameRunningSet::UpdateEntities,
                GameRunningSet::DespawnEntities,
            )
                .chain(),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameRunningSet {
    FetchData,
    GetUserInput,
    SendEvents,
    SendCommands,
    HandleCommands,
    UpdateEntities,
    DespawnEntities,
}
