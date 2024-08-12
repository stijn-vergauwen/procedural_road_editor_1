use std::ops::Range;

use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    ui::{build_text_node, buttons::spawn_button_node},
    utility::entity_is_descendant_of,
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
    for (button_entity, _, number_input_button) in button_query
        .iter_mut()
        .filter(|(_, interaction, _)| **interaction == Interaction::Pressed)
    {
        let (number_input_entity, mut number_input) = number_input_query
            .iter_mut()
            .find(|(entity, _)| entity_is_descendant_of(&parent_query, button_entity, *entity))
            .unwrap();

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
    mut number_display_query: Query<(Entity, &NumberInputDisplay, &mut Text)>,
    parent_query: Query<&Parent>,
) {
    for event in on_changed.read() {
        let (_, _, mut text) = number_display_query.iter_mut()
        .find(|(entity, _, _)| entity_is_descendant_of(&parent_query, *entity, event.number_input_entity()))
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
    let number_input_entity = number_input.id();

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
                    NumberInputButton::new(NumberInputDirection::Down),
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
                            NumberInputDisplay,
                        ));
                    });

                spawn_button_node(
                    elements_container,
                    NumberInputButton::new(NumberInputDirection::Up),
                    ">",
                    20.0,
                );
            });
    });

    number_input_entity
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
    (
        NumberInputDisplay,
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(4.0), Val::Px(2.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor(NEUTRAL_900.into()),
            ..default()
        },
    )
}
