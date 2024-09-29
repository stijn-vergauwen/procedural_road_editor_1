use bevy::prelude::*;

pub struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameMode::MainMenu);
    }
}

#[derive(States, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum GameMode {
    MainMenu,
    RoadEditor,
    RoadDrawer,
}
