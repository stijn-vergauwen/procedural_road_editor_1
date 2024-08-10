use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{utility::texture_builder::TextureBuilder, GameRunningSet};

use super::slider_input::{spawn_slider_input_with_image, OnSliderInputValueChanged};

// TODO: update textures when color changes
// TODO: display current color

pub struct ColorInputPlugin;

impl Plugin for ColorInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnColorInputValueChanged>()
            .add_systems(Startup, spawn_test_thing)
            .add_systems(
                Update,
                send_color_input_changed_events.in_set(GameRunningSet::SendEvents),
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
#[derive(Clone, Copy)]
enum ColorChannel {
    A,
    B,
    C,
    Alpha,
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
    let red_image = images.add(TextureBuilder::image_from_colors(vec![
        LinearRgba::WHITE.into(),
        LinearRgba::RED.into(),
    ]));
    let green_image = images.add(TextureBuilder::image_from_colors(vec![
        LinearRgba::WHITE.into(),
        LinearRgba::GREEN.into(),
    ]));
    let blue_image = images.add(TextureBuilder::image_from_colors(vec![
        LinearRgba::WHITE.into(),
        LinearRgba::BLUE.into(),
    ]));

    let mut color_input = builder.spawn(build_color_input_container_node());
    let color_input_entity = color_input.id();

    color_input.with_children(|color_input| {
        spawn_color_input_slider(color_input, color_input_entity, ColorChannel::A, red_image);
        spawn_color_input_slider(
            color_input,
            color_input_entity,
            ColorChannel::B,
            green_image,
        );
        spawn_color_input_slider(color_input, color_input_entity, ColorChannel::C, blue_image);
    });

    color_input_entity
}

fn spawn_color_input_slider(
    builder: &mut ChildBuilder,
    color_input_entity: Entity,
    color_channel: ColorChannel,
    image: Handle<Image>,
) -> Entity {
    spawn_slider_input_with_image(
        builder,
        ColorInputSlider::new(color_input_entity, color_channel),
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
    color_input_slider_query: Query<&mut UiImage, With<ColorInputSlider>>,
) {
    for event in on_color_changed.read() {
        // TODO: wrong entity is being matched, the query should be that of the color input, not it's sliders
        let Ok(color_input_slider) = color_input_slider_query.get(event.color_input_entity) else {
            continue;
        };

        // TODO: Left off here: implement this method, update the images of the ui sliders (check the entity event for if the input slider is the correct one?)

        // let color_input_entity = color_input_slider.color_input_entity;
        // let color_channel = color_input_slider.color_channel;

        // let Ok(mut color_input) = color_input_query.get_mut(color_input_entity) else {
        //     continue;
        // };

        // let new_color =
        //     get_rgba_color_with_channel(color_input.value, color_channel, event.new_value());

        // color_input.value = new_color;

        // on_color_changed.send(OnColorInputValueChanged::new(
        //     color_input_entity,
        //     color_channel,
        //     new_color,
        // ));
    }
}

// Utility

fn get_rgba_color_with_channel(color: Color, channel: ColorChannel, new_value: f32) -> Color {
    match channel {
        ColorChannel::A => color.to_srgba().with_red(new_value).into(),
        ColorChannel::B => color.to_srgba().with_green(new_value).into(),
        ColorChannel::C => color.to_srgba().with_blue(new_value).into(),
        ColorChannel::Alpha => color.to_srgba().with_alpha(new_value).into(),
    }
}

// Node builders

fn build_color_input_container_node() -> impl Bundle {
    (
        ColorInput::new(Color::WHITE),
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
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
