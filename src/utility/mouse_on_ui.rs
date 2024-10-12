use bevy::prelude::*;

use crate::GameRunningSet;

pub struct MouseOnUiPlugin;

impl Plugin for MouseOnUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseOnUi>()
            .add_systems(Update, update_mouse_on_ui.in_set(GameRunningSet::FetchData));
    }
}

#[derive(Resource, Debug, Default)]
pub struct MouseOnUi {
    is_on_ui: bool,
}

impl MouseOnUi {
    pub fn is_on_ui(&self) -> bool {
        self.is_on_ui
    }
}

fn update_mouse_on_ui(mut mouse_on_ui: ResMut<MouseOnUi>, nodes: Query<&Interaction>) {
    mouse_on_ui.is_on_ui = nodes.iter().any(|interaction| {
        *interaction == Interaction::Hovered || *interaction == Interaction::Pressed
    });
}
