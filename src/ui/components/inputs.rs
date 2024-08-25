pub mod color_input;
pub mod number_input;
pub mod slider_input;

use bevy::prelude::*;
use color_input::ColorInputPlugin;
use number_input::NumberInputPlugin;
use slider_input::SliderInputPlugin;

// TODO: "labeled_element" component, puts a text label above (or next to) the given children

pub struct InputComponentsPlugin;

impl Plugin for InputComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((NumberInputPlugin, SliderInputPlugin, ColorInputPlugin));
    }
}
