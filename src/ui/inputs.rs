pub mod number_input;

use bevy::prelude::*;
use number_input::NumberInputPlugin;

pub struct UiInputsPlugin;

impl Plugin for UiInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NumberInputPlugin);
    }
}
