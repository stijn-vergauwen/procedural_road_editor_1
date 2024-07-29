mod toolbar;

use bevy::prelude::*;
use toolbar::ToolbarPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToolbarPlugin);
    }
}
