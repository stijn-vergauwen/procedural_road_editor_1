pub mod components;
mod inputs;
pub mod list;
mod modal;
mod sidebar;
pub mod toolbar;

use bevy::prelude::*;
use components::{
    flexbox::{FlexboxBuilder, FlexboxConfig},
    UiComponentWithChildrenBuilder, UiComponentsPlugin,
};
use inputs::UiInputsPlugin;
use list::ListPlugin;
use modal::ModalPlugin;
use sidebar::{spawn_sidebar, SidebarPlugin};
use toolbar::{spawn_toolbar, ToolbarPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiComponentsPlugin,
            ToolbarPlugin,
            SidebarPlugin,
            UiInputsPlugin,
            ModalPlugin,
            ListPlugin,
        ))
        .add_systems(Startup, spawn_layout_container);
    }
}

fn spawn_layout_container(mut commands: Commands) {
    let container_node = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            row_gap: Val::Px(20.0),
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    };

    commands.spawn(container_node).with_children(|container| {
        FlexboxBuilder::new(FlexboxConfig::row().with_justify(JustifyContent::Start)).spawn(
            container,
            (),
            |left_side| {
                spawn_sidebar(left_side);
            },
        );

        spawn_toolbar(container);
    });
}

// TODO: delete
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
