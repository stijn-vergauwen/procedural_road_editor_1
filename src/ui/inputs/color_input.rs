use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::utility::texture_builder::TextureBuilder;

use super::slider_input::spawn_slider_input_with_image;

// TODO: sliders for rgb <- doing
// TODO: update textures when color changes
// TODO: display current color

pub struct ColorInputPlugin;

impl Plugin for ColorInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_thing);
    }
}

#[derive(Component)]
struct ColorInput;

fn spawn_test_thing(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands
        .spawn(build_centered_container_node())
        .with_children(|container| {
            container
                .spawn(build_background_section_node())
                .with_children(|section| {
                    spawn_color_input(section, &mut images);
                });
        });
}

pub fn spawn_color_input(builder: &mut ChildBuilder, images: &mut Assets<Image>) -> Entity {
    // Texture test
    let linear_colors = vec![LinearRgba::RED.into(), LinearRgba::BLUE.into()];
    let s_colors = vec![Color::srgb(0.0, 0.5, 0.5), Color::srgb(1.0, 0.5, 0.5)];

    let linear_image = images.add(TextureBuilder::image_from_colors(linear_colors));
    let s_image = images.add(TextureBuilder::image_from_colors(s_colors));
    // End of texture test

    let mut color_input = builder.spawn(build_color_input_container_node());
    let color_input_entity = color_input.id();

    color_input.with_children(|color_input| {
        spawn_slider_input_with_image(color_input, (), linear_image);
        spawn_slider_input_with_image(color_input, (), s_image);
    });

    color_input_entity
}

// Node builders

fn build_color_input_container_node() -> impl Bundle {
    (
        ColorInput,
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: NEUTRAL_600.into(),
            ..default()
        },
    )
}

fn build_centered_container_node() -> impl Bundle {
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

fn build_background_section_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        background_color: NEUTRAL_700.into(),
        ..default()
    }
}
