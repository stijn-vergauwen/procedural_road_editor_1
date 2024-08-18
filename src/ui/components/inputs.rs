pub mod number_input;

use bevy::prelude::*;
use number_input::NumberInputPlugin;

// TODO: "labeled_element" component, puts a text label above (or next to) the given children

pub struct InputComponentsPlugin;

impl Plugin for InputComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NumberInputPlugin);
    }
}
