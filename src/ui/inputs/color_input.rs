use bevy::prelude::*;

pub struct ColorInputPlugin;

impl Plugin for ColorInputPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, ());
    }
}

// TODO: make input container
// TODO: make slider component
// TODO: make slider handle
// TODO: draggable slider handle (0 to 1)
// TODO: split slider to module
// TODO: slider with texture as background
// TODO: generate color texture
// TODO: sliders for rgb
// TODO: update textures when color changes
// TODO: display current color