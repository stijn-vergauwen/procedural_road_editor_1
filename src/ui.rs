pub mod components;
mod inputs;
pub mod list;
mod modal;
mod sidebar;
pub mod toolbar;

use bevy::{color::palettes::tailwind::*, prelude::*};
use components::{
    buttons::ButtonBuilder, section::SectionBuilder, text::SimpleTextBuilder, UiComponentBuilder,
    UiComponentWithChildrenBuilder, UiComponentsPlugin,
};
use inputs::UiInputsPlugin;
use list::ListPlugin;
use modal::ModalPlugin;
use sidebar::SidebarPlugin;
use toolbar::ToolbarPlugin;

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
        .add_systems(Startup, spawn_template_test_thing);
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

// TEST

fn spawn_template_test_thing(mut commands: Commands) {
    commands
        .spawn(centered_container_node())
        .with_children(|container| {
            SectionBuilder::spawn_default(container, (), |section| {
                section.spawn(build_test_content_node());

                ButtonBuilder::default()
                    .with_background_color(BLUE_400)
                    .spawn(section, (), |button| {
                        SimpleTextBuilder::default()
                            .with_text("Test text")
                            .spawn(button, ());
                    });

                section.spawn(build_test_content_node());
            });
        });
}

pub fn centered_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

fn build_test_content_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(40.0),
            ..default()
        },
        background_color: BLUE_500.into(),
        ..default()
    }
}
