mod buttons;
mod inputs;
mod list;
mod modal;
mod sidebar;
pub mod toolbar;

pub use list::{List, ListItem};

use bevy::prelude::*;
use buttons::ButtonsPlugin;
use inputs::UiInputsPlugin;
use modal::ModalPlugin;
use sidebar::SidebarPlugin;
use toolbar::ToolbarPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ButtonsPlugin,
            ToolbarPlugin,
            SidebarPlugin,
            UiInputsPlugin,
            ModalPlugin,
        ));
    }
}

fn build_text_node(
    text: impl Into<String>,
    font_size: f32,
    font_color: impl Into<Color>,
    justify: JustifyText,
    text_components: impl Bundle,
) -> impl Bundle {
    (
        text_components,
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text.into(),
                    style: TextStyle {
                        color: font_color.into(),
                        font_size,
                        ..default()
                    },
                }],
                justify,
                ..default()
            },
            ..default()
        },
    )
}