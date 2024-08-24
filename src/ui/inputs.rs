pub mod color_input;
pub mod text_input;

use bevy::prelude::*;
use color_input::ColorInputPlugin;
use text_input::TextInputPlugin;

pub struct UiInputsPlugin;

impl Plugin for UiInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TextInputPlugin, ColorInputPlugin));
    }
}
