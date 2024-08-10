use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{ui::build_text_node, utility::texture_builder::TextureBuilder, GameRunningSet};

use super::slider_input::{spawn_slider_input_with_image, OnSliderInputValueChanged};

// TODO: display current color

pub struct ColorInputPlugin;

impl Plugin for ColorInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnColorInputValueChanged>()
            .add_systems(Startup, spawn_test_thing)
            .add_systems(
                Update,
                (
                    send_color_input_changed_events.in_set(GameRunningSet::SendEvents),
                    (update_color_input_textures, update_color_input_display)
                        .in_set(GameRunningSet::UpdateEntities),
                ),
            );
    }
}

#[derive(Component)]
struct ColorInput {
    value: Color,
}

impl ColorInput {
    fn new(value: Color) -> Self {
        Self { value }
    }
}

#[derive(Component)]
struct ColorInputSlider {
    color_input_entity: Entity,
    color_channel: ColorChannel,
}

impl ColorInputSlider {
    fn new(color_input_entity: Entity, color_channel: ColorChannel) -> Self {
        Self {
            color_input_entity,
            color_channel,
        }
    }
}

/// The different channels of a full color representation.  
/// For example: red, green, and blue for RGB colors.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum ColorChannel {
    A,
    B,
    C,
    Alpha,
}

#[derive(Component)]
struct ColorInputDisplay {
    color_input_entity: Entity,
}

impl ColorInputDisplay {
    fn new(color_input_entity: Entity) -> Self {
        Self { color_input_entity }
    }
}

#[derive(Event)]
struct OnColorInputValueChanged {
    color_input_entity: Entity,
    color_channel: ColorChannel,
    new_color: Color,
}

impl OnColorInputValueChanged {
    fn new(color_input_entity: Entity, color_channel: ColorChannel, new_color: Color) -> Self {
        Self {
            color_input_entity,
            color_channel,
            new_color,
        }
    }
}

fn spawn_test_thing(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let start_color = Color::srgb(0.4, 0.3, 0.8);

    commands
        .spawn(build_centered_container_node())
        .with_children(|container| {
            container
                .spawn(build_background_section_node())
                .with_children(|section| {
                    spawn_color_input(section, &mut images, start_color, Some("Color"));
                });
        });
}

pub fn spawn_color_input(
    builder: &mut ChildBuilder,
    images: &mut Assets<Image>,
    start_color: Color,
    label: Option<impl Into<String>>,
) -> Entity {
    let mut color_input = builder.spawn(build_color_input_container_node());
    let color_input_entity = color_input.id();

    color_input.with_children(|color_input| {
        if let Some(label) = label {
            color_input.spawn(build_text_node(
                label,
                20.0,
                Color::WHITE,
                JustifyText::Center,
                (),
            ));
        }

        color_input.spawn(build_color_display_node(color_input_entity, start_color));

        spawn_color_input_slider(
            color_input,
            images,
            color_input_entity,
            start_color,
            ColorChannel::A,
        );

        spawn_color_input_slider(
            color_input,
            images,
            color_input_entity,
            start_color,
            ColorChannel::B,
        );

        spawn_color_input_slider(
            color_input,
            images,
            color_input_entity,
            start_color,
            ColorChannel::C,
        );
    });

    color_input_entity
}

fn spawn_color_input_slider(
    builder: &mut ChildBuilder,
    images: &mut Assets<Image>,
    color_input_entity: Entity,
    start_color: Color,
    color_channel: ColorChannel,
) -> Entity {
    let image = images.add(generate_slider_image(start_color, color_channel));

    spawn_slider_input_with_image(
        builder,
        ColorInputSlider::new(color_input_entity, color_channel),
        get_rgba_color_channel(start_color, color_channel),
        image,
    )
}

fn send_color_input_changed_events(
    mut on_color_changed: EventWriter<OnColorInputValueChanged>,
    mut on_slider_changed: EventReader<OnSliderInputValueChanged>,
    color_input_slider_query: Query<&ColorInputSlider>,
    mut color_input_query: Query<&mut ColorInput>,
) {
    for event in on_slider_changed.read() {
        let Ok(color_input_slider) = color_input_slider_query.get(event.slider_input_entity())
        else {
            continue;
        };

        let color_input_entity = color_input_slider.color_input_entity;
        let color_channel = color_input_slider.color_channel;

        let Ok(mut color_input) = color_input_query.get_mut(color_input_entity) else {
            continue;
        };

        let new_color =
            get_rgba_color_with_channel(color_input.value, color_channel, event.new_value());

        color_input.value = new_color;

        on_color_changed.send(OnColorInputValueChanged::new(
            color_input_entity,
            color_channel,
            new_color,
        ));
    }
}

fn update_color_input_textures(
    mut on_color_changed: EventReader<OnColorInputValueChanged>,
    mut images: ResMut<Assets<Image>>,
    color_input_slider_query: Query<(&UiImage, &ColorInputSlider)>,
) {
    for event in on_color_changed.read() {
        for (ui_image, slider) in color_input_slider_query.iter().filter(|(_, slider)| {
            slider.color_input_entity == event.color_input_entity
                && slider.color_channel != event.color_channel
        }) {
            let image = images.get_mut(&ui_image.texture).unwrap();

            *image = generate_slider_image(event.new_color, slider.color_channel);
        }
    }
}

fn update_color_input_display(
    mut on_color_changed: EventReader<OnColorInputValueChanged>,
    mut color_input_display_query: Query<(&mut BackgroundColor, &ColorInputDisplay)>,
) {
    for event in on_color_changed.read() {
        for (mut background_color, _) in color_input_display_query
            .iter_mut()
            .filter(|(_, display)| display.color_input_entity == event.color_input_entity)
        {
            *background_color = event.new_color.into();
        }
    }
}

// Utility

fn generate_slider_image(color: Color, channel: ColorChannel) -> Image {
    TextureBuilder::image_from_colors(vec![
        get_rgba_color_with_channel(color, channel, 0.0),
        get_rgba_color_with_channel(color, channel, 1.0),
    ])
}

fn get_rgba_color_with_channel(color: Color, channel: ColorChannel, new_value: f32) -> Color {
    match channel {
        ColorChannel::A => color.to_srgba().with_red(new_value).into(),
        ColorChannel::B => color.to_srgba().with_green(new_value).into(),
        ColorChannel::C => color.to_srgba().with_blue(new_value).into(),
        ColorChannel::Alpha => color.to_srgba().with_alpha(new_value).into(),
    }
}

fn get_rgba_color_channel(color: Color, channel: ColorChannel) -> f32 {
    match channel {
        ColorChannel::A => color.to_srgba().red,
        ColorChannel::B => color.to_srgba().green,
        ColorChannel::C => color.to_srgba().blue,
        ColorChannel::Alpha => color.to_srgba().alpha,
    }
}

// Node builders

fn build_color_input_container_node() -> impl Bundle {
    (
        ColorInput::new(Color::WHITE),
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(4.0)),
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

fn build_color_display_node(color_input_entity: Entity, start_color: Color) -> impl Bundle {
    (
        ColorInputDisplay::new(color_input_entity),
        NodeBundle {
            style: Style {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            border_color: NEUTRAL_700.into(),
            background_color: start_color.into(),
            ..default()
        },
    )
}
