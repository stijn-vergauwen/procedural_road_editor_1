use std::ops::Range;

use bevy::{prelude::*, text::BreakLineOn};

use crate::{
    ui::components::{
        buttons::ButtonBuilder,
        content_wrap::ContentWrapConfig,
        flexbox::FlexboxConfig,
        section::{SectionBuilder, SectionConfig},
        text::{TextBuilder, TextConfig},
        UiComponentBuilder, UiComponentWithChildrenBuilder,
    },
    utility::{entity_is_descendant_of, find_ancestor_of_entity_mut},
    GameRunningSet,
};

pub struct NumberInputPlugin;

impl Plugin for NumberInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnNumberInputValueChanged>().add_systems(
            Update,
            (
                update_number_input_value_on_button_press.in_set(GameRunningSet::GetUserInput),
                update_number_display.in_set(GameRunningSet::UpdateEntities),
            ),
        );
    }
}

// Start of new UiComponent code

// TODO: make generic text input UiComponent
// TODO: make number input build on top of the text input UiComponent, (so it also supports keyboard input, but limited to numbers within a range)

#[derive(Clone, Copy)]
pub struct NumberInputDisplayConfig {
    pub color: Color,
    pub font_size: f32,
    pub justify: JustifyText,
    pub linebreak_behavior: BreakLineOn,
}

impl Default for NumberInputDisplayConfig {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            font_size: 24.0,
            justify: JustifyText::Center,
            linebreak_behavior: BreakLineOn::NoWrap,
        }
    }
}

impl From<NumberInputDisplayConfig> for TextConfig {
    fn from(value: NumberInputDisplayConfig) -> Self {
        TextConfig {
            color: value.color,
            font_size: value.font_size,
            justify: value.justify,
            linebreak_behavior: value.linebreak_behavior,
            ..default()
        }
    }
}

#[derive(Clone)]
pub struct NumberInputButtonConfig {
    pub text: String,
    pub color: Color,
    pub font_size: f32,
}

impl NumberInputButtonConfig {
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl Default for NumberInputButtonConfig {
    fn default() -> Self {
        Self {
            text: String::new(),
            color: Color::WHITE,
            font_size: 24.0,
        }
    }
}

impl From<NumberInputButtonConfig> for TextConfig {
    fn from(value: NumberInputButtonConfig) -> Self {
        TextConfig {
            text: value.text,
            color: value.color,
            font_size: value.font_size,
            ..default()
        }
    }
}

#[derive(Clone)]
pub struct NumberInputConfig {
    start_value: f32,
    // TODO: change to inclusive range
    value_range: Range<f32>,
    display: NumberInputDisplayConfig,
    up_button: NumberInputButtonConfig,
    down_button: NumberInputButtonConfig,
}

impl Default for NumberInputConfig {
    fn default() -> Self {
        Self {
            start_value: 0.0,
            value_range: 0.0..1.0,
            display: NumberInputDisplayConfig::default(),
            up_button: NumberInputButtonConfig::default().with_text(">"),
            down_button: NumberInputButtonConfig::default().with_text("<"),
        }
    }
}

/// A number UiComponent with text as content.
#[derive(Default)]
pub struct NumberInputBuilder {
    config: NumberInputConfig,
}

impl NumberInputBuilder {
    pub fn new(config: NumberInputConfig) -> Self {
        Self { config }
    }

    pub fn with_values(&mut self, start_value: f32, value_range: Range<f32>) -> &mut Self {
        self.config.start_value = start_value;
        self.config.value_range = value_range;
        self
    }
}

impl UiComponentBuilder for NumberInputBuilder {
    fn spawn(&self, builder: &mut ChildBuilder, components: impl Bundle) -> Entity {
        SectionBuilder::new(SectionConfig::empty().with_full_width()).spawn(
            builder,
            (components, self.build()),
            |number_input| {
                // Down button
                ButtonBuilder::spawn_default(
                    number_input,
                    NumberInputButton::new(NumberInputDirection::Down),
                    |down_button| {
                        TextBuilder::new(TextConfig::from(self.config.down_button.clone()))
                            .spawn(down_button, ());
                    },
                );

                // Number input display
                SectionBuilder::new(SectionConfig {
                    wrap: ContentWrapConfig::empty().with_full_width(),
                    flexbox: FlexboxConfig::centered(),
                })
                .spawn(number_input, (), |display_wrap| {
                    TextBuilder::new(TextConfig::from(self.config.display))
                        .with_text(format_display_value(self.config.start_value))
                        .spawn(display_wrap, NumberInputDisplay);
                });

                // Up button
                ButtonBuilder::spawn_default(
                    number_input,
                    NumberInputButton::new(NumberInputDirection::Up),
                    |up_button| {
                        TextBuilder::new(TextConfig::from(self.config.up_button.clone()))
                            .spawn(up_button, ());
                    },
                );
            },
        )
    }

    fn build(&self) -> impl Bundle {
        NumberInput::new(self.config.start_value, self.config.value_range.clone())
    }
}

// End of new UiComponent code

#[derive(Component)]
pub struct NumberInput {
    value: f32,
    allowed_range: Range<f32>,
}

impl NumberInput {
    pub fn new(value: f32, allowed_range: Range<f32>) -> Self {
        Self {
            value,
            allowed_range,
        }
    }

    pub fn change_value(&mut self, delta_value: f32) {
        let new_value = self.value + delta_value;
        let clamped = new_value.clamp(self.allowed_range.start, self.allowed_range.end);

        self.value = clamped;
    }
}

#[derive(Component)]
pub struct NumberInputButton {
    direction: NumberInputDirection,
}

impl NumberInputButton {
    pub fn new(direction: NumberInputDirection) -> Self {
        Self { direction }
    }
}

#[derive(Component)]
pub struct NumberInputDisplay;

#[derive(Clone, Copy)]
pub enum NumberInputDirection {
    Up,
    Down,
}

#[derive(Event)]
pub struct OnNumberInputValueChanged {
    number_input_entity: Entity,
    new_value: f32,
}

impl OnNumberInputValueChanged {
    pub fn new(number_input_entity: Entity, new_value: f32) -> Self {
        Self {
            number_input_entity,
            new_value,
        }
    }

    pub fn number_input_entity(&self) -> Entity {
        self.number_input_entity
    }

    pub fn new_value(&self) -> f32 {
        self.new_value
    }
}

fn update_number_input_value_on_button_press(
    mut on_changed: EventWriter<OnNumberInputValueChanged>,
    mut button_query: Query<(Entity, &Interaction, &mut NumberInputButton), Changed<Interaction>>,
    mut number_input_query: Query<(Entity, &mut NumberInput)>,
    parent_query: Query<&Parent>,
) {
    for (button_entity, _, button) in button_query
        .iter_mut()
        .filter(|(_, interaction, _)| **interaction == Interaction::Pressed)
    {
        let Some((number_input_entity, mut number_input)) = find_ancestor_of_entity_mut(
            button_entity,
            &mut number_input_query,
            |item| item.0,
            &parent_query,
        ) else {
            continue;
        };

        let delta_value = match button.direction {
            NumberInputDirection::Up => 0.1,
            NumberInputDirection::Down => -0.1,
        };

        number_input.change_value(delta_value);

        on_changed.send(OnNumberInputValueChanged::new(
            number_input_entity,
            number_input.value,
        ));
    }
}

fn update_number_display(
    mut on_changed: EventReader<OnNumberInputValueChanged>,
    mut number_display_query: Query<(Entity, &NumberInputDisplay, &mut Text)>,
    parent_query: Query<&Parent>,
) {
    for event in on_changed.read() {
        // TODO: replace with utility method
        let (_, _, mut text) = number_display_query.iter_mut()
        .find(|(entity, _, _)| entity_is_descendant_of(&parent_query, *entity, event.number_input_entity()))
        .expect("NumberInputValueChanged event should always match NumberInput entity with a display node.");

        text.sections[0].value = format_display_value(event.new_value());
    }
}

fn format_display_value(value: f32) -> String {
    format!("{:.1}", value)
}
