use std::ops::Range;

use bevy::{color::palettes::tailwind::*, prelude::*, text::BreakLineOn};

use crate::{
    ui::{
        build_text_node,
        components::{
            buttons::{spawn_button_node, ButtonBuilder},
            content_wrap::ContentWrapBuilder,
            flexbox::FlexboxBuilder,
            text::{SimpleTextBuilder, SimpleTextConfig},
            UiComponentBuilder, UiComponentWithChildrenBuilder,
        },
    },
    utility::partial::Partial,
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
// TODO: make number input build on top of the text input UiComponent, (so it also supports keyboard input, but limits it to numbers within a range)

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

impl From<NumberInputDisplayConfig> for SimpleTextConfig {
    fn from(value: NumberInputDisplayConfig) -> Self {
        SimpleTextConfig {
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

impl From<NumberInputButtonConfig> for SimpleTextConfig {
    fn from(value: NumberInputButtonConfig) -> Self {
        SimpleTextConfig {
            text: value.text,
            color: value.color,
            font_size: value.font_size,
            ..default()
        }
    }
}

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
}

impl UiComponentBuilder for NumberInputBuilder {
    fn spawn(&self, builder: &mut ChildBuilder, components: impl Bundle) -> Entity {
        FlexboxBuilder::spawn_default(builder, (components, self.build()), |number_input| {
            // Down button
            ButtonBuilder::spawn_default(
                number_input,
                NumberInputButton::new(NumberInputDirection::Down),
                |down_button| {
                    SimpleTextBuilder::new(SimpleTextConfig::from(self.config.down_button.clone()))
                        .spawn(down_button, ());
                },
            );

            // Number input display
            ContentWrapBuilder::spawn_default(number_input, (), |display_wrap| {
                SimpleTextBuilder::new(SimpleTextConfig::from(self.config.display))
                    .spawn(display_wrap, NumberInputDisplay);
            });

            // Up button
            ButtonBuilder::spawn_default(
                number_input,
                NumberInputButton::new(NumberInputDirection::Up),
                |up_button| {
                    SimpleTextBuilder::new(SimpleTextConfig::from(self.config.up_button.clone()))
                        .spawn(up_button, ());
                },
            );
        })
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
    mut button_query: Query<(&Interaction, &mut NumberInputButton, &Partial), Changed<Interaction>>,
    mut number_input_query: Query<(Entity, &mut NumberInput)>,
) {
    for (_, number_input_button, partial) in button_query
        .iter_mut()
        .filter(|(interaction, _, _)| **interaction == Interaction::Pressed)
    {
        let (number_input_entity, mut number_input) =
            number_input_query.get_mut(partial.main_entity()).unwrap();

        let delta_value = match number_input_button.direction {
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
    mut number_display_query: Query<(&NumberInputDisplay, &mut Text, &Partial)>,
) {
    for event in on_changed.read() {
        let (_, mut text, _) = number_display_query.iter_mut()
        .find(|(_, _, partial)| partial.main_entity() == event.number_input_entity())
        .expect("NumberInputValueChanged event should always match NumberInput entity with a display node.");

        text.sections[0].value = format_display_value(event.new_value());
    }
}

pub fn spawn_number_input_node(
    builder: &mut ChildBuilder,
    root_components: impl Bundle,
    label: impl Into<String>,
    start_value: f32,
    value_range: Range<f32>,
) -> Entity {
    let mut number_input = builder.spawn(build_number_input_container_node(
        root_components,
        start_value,
        value_range,
    ));
    let main_entity = number_input.id();

    number_input.with_children(|number_input| {
        number_input.spawn(build_text_node(
            label,
            20.0,
            Color::WHITE,
            JustifyText::Center,
            (),
        ));

        number_input
            .spawn(build_number_input_elements_container_node())
            .with_children(|elements_container| {
                spawn_button_node(
                    elements_container,
                    (
                        NumberInputButton::new(NumberInputDirection::Down),
                        Partial::new(main_entity),
                    ),
                    "<",
                    20.0,
                );

                elements_container
                    .spawn(build_numer_input_display_node())
                    .with_children(|number_display| {
                        number_display.spawn(build_text_node(
                            format_display_value(start_value),
                            20.0,
                            Color::WHITE,
                            JustifyText::Center,
                            (NumberInputDisplay, Partial::new(main_entity)),
                        ));
                    });

                spawn_button_node(
                    elements_container,
                    (
                        NumberInputButton::new(NumberInputDirection::Up),
                        Partial::new(main_entity),
                    ),
                    ">",
                    20.0,
                );
            });
    });

    main_entity
}

fn format_display_value(value: f32) -> String {
    format!("{:.1}", value)
}

fn build_number_input_container_node(
    root_components: impl Bundle,
    start_value: f32,
    value_range: Range<f32>,
) -> impl Bundle {
    (
        root_components,
        NumberInput::new(start_value, value_range),
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
    )
}

fn build_number_input_elements_container_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(4.0),
            ..default()
        },
        ..default()
    }
}

fn build_numer_input_display_node() -> impl Bundle {
    (NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(4.0), Val::Px(2.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        border_color: BorderColor(NEUTRAL_900.into()),
        ..default()
    },)
}
