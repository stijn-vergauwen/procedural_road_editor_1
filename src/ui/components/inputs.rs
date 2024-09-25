pub mod color_input;
pub mod number_input;
pub mod slider_input;
pub mod labeled_element;

use bevy::prelude::*;
use color_input::ColorInputPlugin;
use number_input::NumberInputPlugin;
use slider_input::SliderInputPlugin;

pub struct InputComponentsPlugin;

impl Plugin for InputComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((NumberInputPlugin, SliderInputPlugin, ColorInputPlugin));
    }
}
