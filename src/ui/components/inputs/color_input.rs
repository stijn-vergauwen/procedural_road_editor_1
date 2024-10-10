use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    ui::components::{
        content_size::ContentSizeConfig,
        content_wrap::ContentWrapConfig,
        flexbox::FlexboxConfig,
        inputs::slider_input::{OnSliderInputValueChanged, SliderInputBuilder, SliderInputConfig},
        section::{SectionBuilder, SectionConfig},
        UiComponentBuilder, UiComponentWithChildrenBuilder,
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

// Start of new UiComponent code

#[derive(Clone, Copy)]
pub struct ColorInputConfig {
    start_color: Color,
    section: SectionConfig,
}

impl ColorInputConfig {
    pub fn with_start_color(mut self, start_color: impl Into<Color>) -> Self {
        self.start_color = start_color.into();
        self
    }
}

impl Default for ColorInputConfig {
    fn default() -> Self {
        Self {
            start_color: Color::WHITE,
            section: SectionConfig {
                wrap: ContentWrapConfig::default()
                    .with_all_px_border_radius(8.0)
                    .with_background_color(NEUTRAL_500),
                flexbox: FlexboxConfig::horizontally_centered_column().with_px_gap(4.0),
                size: ContentSizeConfig::empty()
                    .with_full_width()
                    .with_min_width(160.0),
            },
        }
    }
}

/// A color selector UiComponent.
#[derive(Default)]
pub struct ColorInputBuilder {
    slider_image_a: Handle<Image>,
    slider_image_b: Handle<Image>,
    slider_image_c: Handle<Image>,
    config: ColorInputConfig,
}

impl ColorInputBuilder {
    pub fn new(config: ColorInputConfig, images: &mut Assets<Image>) -> Self {
        Self {
            slider_image_a: images.add(generate_slider_image(config.start_color, ColorChannel::A)),
            slider_image_b: images.add(generate_slider_image(config.start_color, ColorChannel::B)),
            slider_image_c: images.add(generate_slider_image(config.start_color, ColorChannel::C)),
            config,
        }
    }

    fn get_color_input_display_config(&self) -> SectionConfig {
        SectionConfig {
            wrap: ContentWrapConfig {
                background_color: self.config.start_color.into(),
                border_size: UiRect::all(Val::Px(4.0)),
                border_color: NEUTRAL_700.into(),
                ..ContentWrapConfig::empty()
            },
            flexbox: FlexboxConfig::default(),
            size: ContentSizeConfig::empty()
                .with_min_width(40.0)
                .with_min_height(40.0),
        }
    }
}

impl UiComponentBuilder for ColorInputBuilder {
    fn spawn(&self, builder: &mut ChildBuilder, components: impl Bundle) -> Entity {
        SectionBuilder::new(self.config.section).spawn(
            builder,
            (components, self.build()),
            |color_input| {
                SectionBuilder::new(self.get_color_input_display_config()).spawn(
                    color_input,
                    ColorInputDisplay,
                    |_| {},
                );

                spawn_slider_input(
                    color_input,
                    self.config.start_color,
                    ColorChannel::A,
                    self.slider_image_a.clone(),
                );

                spawn_slider_input(
                    color_input,
                    self.config.start_color,
                    ColorChannel::B,
                    self.slider_image_b.clone(),
                );

                spawn_slider_input(
                    color_input,
                    self.config.start_color,
                    ColorChannel::C,
                    self.slider_image_c.clone(),
                );
            },
        )
    }

    fn build(&self) -> impl Bundle {
        ColorInput::new(self.config.start_color)
    }
}

// End of new UiComponent code

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

fn spawn_slider_input(
    builder: &mut ChildBuilder,
    start_color: Color,
    color_channel: ColorChannel,
    slider_image: Handle<Image>,
) {
    SliderInputBuilder::new(
        SliderInputConfig::default()
            .with_start_value(get_rgba_color_channel(start_color, color_channel))
            .with_background_image(slider_image)
            .clone(),
    )
    .spawn(builder, ColorInputSlider::new(color_channel));
}

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
