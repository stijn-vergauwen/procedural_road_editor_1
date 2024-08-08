use bevy::{color::palettes::tailwind::*, prelude::*};

// TODO: draggable slider handle (0 to 1)
// TODO: split slider to module
// TODO: slider with texture as background
// TODO: generate color texture
// TODO: sliders for rgb
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

#[derive(Component)]
struct SliderInput;

#[derive(Component)]
struct SliderHandle;

fn spawn_test_thing(mut commands: Commands) {
    commands
        .spawn(build_centered_container_node())
        .with_children(|container| {
            container
                .spawn(build_background_section_node())
                .with_children(|section| {
                    spawn_color_input(section);
                });
        });
}

pub fn spawn_color_input(builder: &mut ChildBuilder) -> Entity {
    let mut color_input = builder.spawn(build_color_input_container_node());
    let color_input_entity = color_input.id();

    color_input.with_children(|color_input| {
        spawn_slider_input(color_input);
    });

    color_input_entity
}

pub fn spawn_slider_input(builder: &mut ChildBuilder) -> Entity {
    let mut slider_input = builder.spawn(build_slider_input_container_node(
        Val::Px(200.0),
        Val::Px(20.0),
    ));
    let slider_input_entity = slider_input.id();

    slider_input.with_children(|color_input| {
        color_input.spawn(build_slider_handle_node());
    });

    slider_input_entity
}

// Node builders

fn build_slider_input_container_node(width: Val, height: Val) -> impl Bundle {
    (
        SliderInput,
        NodeBundle {
            style: Style {
                width,
                height,
                ..default()
            },
            background_color: NEUTRAL_500.into(),
            ..default()
        },
    )
}

fn build_slider_handle_node() -> impl Bundle {
    (
        SliderHandle,
        ButtonBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Px(8.0),
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(4.0)),
            background_color: NEUTRAL_800.into(),
            ..default()
        },
    )
}

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
