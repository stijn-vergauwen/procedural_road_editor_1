pub mod color_input;
pub mod number_input;
pub mod slider_input;
pub mod text_input;

use bevy::prelude::*;
use color_input::ColorInputPlugin;
use number_input::NumberInputPlugin;
use slider_input::SliderInputPlugin;
use text_input::TextInputPlugin;

pub struct UiInputsPlugin;

impl Plugin for UiInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            NumberInputPlugin,
            TextInputPlugin,
            ColorInputPlugin,
            SliderInputPlugin,
        ));
    }
}
