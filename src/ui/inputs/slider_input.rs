use bevy::{color::palettes::tailwind::*, prelude::*, ui::RelativeCursorPosition};

use crate::GameRunningSet;

pub struct SliderInputPlugin;

impl Plugin for SliderInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSliderInputValueChanged>().add_systems(
            Update,
            handle_slider_interaction.in_set(GameRunningSet::UpdateEntities),
        );
    }
}

#[derive(Component)]
pub struct SliderInput {
    value: f32,
}

impl SliderInput {
    fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn value_as_percentage(&self) -> f32 {
        self.value * 100.0
    }
}

#[derive(Component)]
struct SliderHandle {
    slider_input_entity: Entity,
}

#[derive(Event)]
pub struct OnSliderInputValueChanged {
    slider_input_entity: Entity,
    new_value: f32,
}

impl OnSliderInputValueChanged {
    pub fn new(slider_input_entity: Entity, new_value: f32) -> Self {
        Self {
            slider_input_entity,
            new_value,
        }
    }

    pub fn slider_input_entity(&self) -> Entity {
        self.slider_input_entity
    }

    pub fn new_value(&self) -> f32 {
        self.new_value
    }
}

#[allow(unused)]
pub fn spawn_slider_input(builder: &mut ChildBuilder, root_components: impl Bundle) -> Entity {
    let mut slider_input = builder.spawn(build_slider_input_node(
        root_components,
        build_button_bundle(Val::Px(200.0), Val::Px(12.0)),
    ));
    let slider_input_entity = slider_input.id();

    slider_input.with_children(|color_input| {
        color_input
            .spawn(build_slider_handle_node(slider_input_entity))
            .with_children(|slider_handle| {
                slider_handle.spawn(build_slider_handle_bar_node(6.0));
            });
    });

    slider_input_entity
}

#[allow(unused)]
pub fn spawn_slider_input_with_image(
    builder: &mut ChildBuilder,
    root_components: impl Bundle,
    image: Handle<Image>,
) -> Entity {
    let mut slider_input = builder.spawn(build_slider_input_node(
        root_components,
        build_button_bundle_with_image(Val::Px(200.0), Val::Px(12.0), image),
    ));
    let slider_input_entity = slider_input.id();

    slider_input.with_children(|color_input| {
        color_input
            .spawn(build_slider_handle_node(slider_input_entity))
            .with_children(|slider_handle| {
                slider_handle.spawn(build_slider_handle_bar_node(6.0));
            });
    });

    slider_input_entity
}

// TODO: split to "send events" and "update slider", so the event can be sent in the "GetUserInput" set
fn handle_slider_interaction(
    mut on_changed: EventWriter<OnSliderInputValueChanged>,
    mut slider_query: Query<(
        &Interaction,
        &RelativeCursorPosition,
        &mut SliderInput,
        Entity,
    )>,
    mut slider_handle_query: Query<(&SliderHandle, &mut Style)>,
) {
    for (_, relative_cursor_position, mut slider_input, slider_entity) in slider_query
        .iter_mut()
        .filter(|slider| *slider.0 == Interaction::Pressed)
    {
        let Some(new_value) = calculate_slider_value(relative_cursor_position) else {
            continue;
        };

        if new_value == slider_input.value {
            continue;
        }

        let Some((_, mut handle_style)) = slider_handle_query
            .iter_mut()
            .find(|(slider_handle, _)| slider_handle.slider_input_entity == slider_entity)
        else {
            continue;
        };

        update_slider_input(&mut slider_input, &mut handle_style, new_value);

        on_changed.send(OnSliderInputValueChanged::new(
            slider_entity,
            slider_input.value,
        ));
    }
}

fn update_slider_input(slider_input: &mut SliderInput, handle_style: &mut Style, new_value: f32) {
    slider_input.value = new_value;
    handle_style.margin = calculate_handle_margin(&slider_input);
}

fn calculate_handle_margin(slider_input: &SliderInput) -> UiRect {
    UiRect::left(Val::Percent(slider_input.value_as_percentage()))
}

fn calculate_slider_value(relative_cursor_position: &RelativeCursorPosition) -> Option<f32> {
    Some(relative_cursor_position.normalized?.x.clamp(0.0, 1.0))
}

// Node builders

fn build_slider_input_node(components: impl Bundle, button_bundle: ButtonBundle) -> impl Bundle {
    (
        components,
        SliderInput::new(0.0),
        RelativeCursorPosition::default(),
        button_bundle,
    )
}

fn build_slider_handle_node(slider_input_entity: Entity) -> impl Bundle {
    (
        SliderHandle {
            slider_input_entity,
        },
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
    )
}

fn build_slider_handle_bar_node(width: f32) -> impl Bundle {
    (NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            height: Val::Percent(140.0),
            width: Val::Px(width),
            left: Val::Px(width / -2.0),
            top: Val::Percent(-20.0),
            ..default()
        },
        border_radius: BorderRadius::MAX,
        background_color: NEUTRAL_100.into(),
        ..default()
    },)
}

fn build_button_bundle(width: Val, height: Val) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width,
            height,
            ..default()
        },
        background_color: NEUTRAL_500.into(),
        ..default()
    }
}

fn build_button_bundle_with_image(width: Val, height: Val, image: Handle<Image>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width,
            height,
            ..default()
        },
        image: UiImage {
            texture: image,
            ..default()
        },
        ..default()
    }
}
