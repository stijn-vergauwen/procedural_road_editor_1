mod buttons;
mod toolbar;
mod list;

pub use list::{List, ListItem};

use bevy::prelude::*;
use buttons::ButtonsPlugin;
use toolbar::ToolbarPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ButtonsPlugin, ToolbarPlugin));
    }
}