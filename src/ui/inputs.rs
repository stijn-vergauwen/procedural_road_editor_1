pub mod number_input;
pub mod text_input;

use bevy::prelude::*;
use number_input::NumberInputPlugin;
use text_input::TextInputPlugin;

pub struct UiInputsPlugin;

impl Plugin for UiInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((NumberInputPlugin, TextInputPlugin));
    }
}
