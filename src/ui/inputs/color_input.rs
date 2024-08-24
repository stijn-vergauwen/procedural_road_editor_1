use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    ui::{
        build_text_node,
        components::{
            inputs::slider_input::{
                OnSliderInputValueChanged, SliderInputBuilder, SliderInputConfig,
            },
            UiComponentBuilder,
        },
    },
    utility::{
        filter_descendants_of_entity, find_ancestor_of_entity_mut, find_descendant_of_entity_mut,
        texture_builder::TextureBuilder,
    },
    GameRunningSet,
};

pub struct ColorInputPlugin;

impl Plugin for ColorInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnColorInputValueChanged>().add_systems(
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
pub struct ColorInput {
    value: Color,
}

impl ColorInput {
    fn new(value: Color) -> Self {
        Self { value }
    }
}

#[derive(Component)]
struct ColorInputSlider {
    color_channel: ColorChannel,
}

impl ColorInputSlider {
    fn new(color_channel: ColorChannel) -> Self {
        Self { color_channel }
    }
}

/// The different channels of a full color representation.  
/// For example: red, green, and blue for RGB colors.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum ColorChannel {
    A,
    B,
    C,
}

#[derive(Component)]
struct ColorInputDisplay;

#[derive(Event)]
pub struct OnColorInputValueChanged {
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

    pub fn color_input_entity(&self) -> Entity {
        self.color_input_entity
    }

    pub fn new_color(&self) -> Color {
        self.new_color
    }
}

pub fn spawn_color_input(
    builder: &mut ChildBuilder,
    root_components: impl Bundle,
    images: &mut Assets<Image>,
    start_color: Color,
    label: Option<impl Into<String>>,
) -> Entity {
    let mut color_input = builder.spawn(build_color_input_container_node(
        root_components,
        start_color,
    ));

    color_input
        .with_children(|color_input| {
            if let Some(label) = label {
                color_input.spawn(build_text_node(
                    label,
                    20.0,
                    Color::WHITE,
                    JustifyText::Center,
                    (),
                ));
            }

            color_input.spawn(build_color_display_node(start_color));
            spawn_color_input_slider(color_input, images, start_color, ColorChannel::A);
            spawn_color_input_slider(color_input, images, start_color, ColorChannel::B);
            spawn_color_input_slider(color_input, images, start_color, ColorChannel::C);
        })
        .id()
}

fn spawn_color_input_slider(
    builder: &mut ChildBuilder,
    images: &mut Assets<Image>,
    start_color: Color,
    color_channel: ColorChannel,
) -> Entity {
    let image = images.add(generate_slider_image(start_color, color_channel));
    let start_value = get_rgba_color_channel(start_color, color_channel);

    SliderInputBuilder::new(
        SliderInputConfig::default()
            .with_start_value(start_value)
            .with_background_image(image)
            // TODO: why does min_width not have any effect?
            .with_min_width(Val::Px(300.0))
            .clone(),
    )
    .spawn(builder, ColorInputSlider::new(color_channel))
}

fn send_color_input_changed_events(
    mut on_slider_changed: EventReader<OnSliderInputValueChanged>,
    mut on_color_changed: EventWriter<OnColorInputValueChanged>,
    color_input_slider_query: Query<(Entity, &ColorInputSlider)>,
    mut color_input_query: Query<(Entity, &mut ColorInput)>,
    parent_query: Query<&Parent>,
) {
    for event in on_slider_changed.read() {
        let Ok((color_input_slider_entity, color_input_slider)) =
            color_input_slider_query.get(event.slider_input_entity())
        else {
            continue;
        };

        let color_channel = color_input_slider.color_channel;
        let (color_input_entity, mut color_input) = find_ancestor_of_entity_mut(
            color_input_slider_entity,
            &mut color_input_query,
            |item| item.0,
            &parent_query,
        )
        .unwrap();

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
    color_input_slider_query: Query<(Entity, &UiImage, &ColorInputSlider)>,
    children_query: Query<&Children>,
) {
    for event in on_color_changed.read() {
        for (_, ui_image, slider) in filter_descendants_of_entity(
            event.color_input_entity(),
            &color_input_slider_query,
            |item| item.0,
            &children_query,
        )
        .into_iter()
        .filter(|(_, _, slider)| slider.color_channel != event.color_channel)
        {
            let image = images.get_mut(&ui_image.texture).unwrap();

            *image = generate_slider_image(event.new_color, slider.color_channel);
        }
    }
}

fn update_color_input_display(
    mut on_color_changed: EventReader<OnColorInputValueChanged>,
    mut color_input_display_query: Query<(Entity, &mut BackgroundColor), With<ColorInputDisplay>>,
    children_query: Query<&Children>,
) {
    for event in on_color_changed.read() {
        let (_, mut background_color) = find_descendant_of_entity_mut(
            event.color_input_entity(),
            &mut color_input_display_query,
            |item| item.0,
            &children_query,
        )
        .unwrap();

        *background_color = event.new_color.into();
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
    }
}

fn get_rgba_color_channel(color: Color, channel: ColorChannel) -> f32 {
    match channel {
        ColorChannel::A => color.to_srgba().red,
        ColorChannel::B => color.to_srgba().green,
        ColorChannel::C => color.to_srgba().blue,
    }
}

// Node builders

fn build_color_input_container_node(
    root_components: impl Bundle,
    start_color: Color,
) -> impl Bundle {
    (
        root_components,
        ColorInput::new(start_color),
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

fn build_color_display_node(start_color: Color) -> impl Bundle {
    (
        ColorInputDisplay,
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
