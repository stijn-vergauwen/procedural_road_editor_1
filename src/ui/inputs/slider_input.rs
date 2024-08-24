use bevy::{color::palettes::tailwind::*, prelude::*, ui::RelativeCursorPosition};

use crate::{utility::find_descendant_of_entity_mut, GameRunningSet};

pub struct SliderInputPlugin;

impl Plugin for SliderInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSliderInputValueChanged>().add_systems(
            Update,
            (
                send_slider_value_changed_events.in_set(GameRunningSet::GetUserInput),
                update_slider_handle_position.in_set(GameRunningSet::UpdateEntities),
            ),
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
struct SliderHandle;

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
pub fn spawn_slider_input(
    builder: &mut ChildBuilder,
    root_components: impl Bundle,
    start_value: f32,
) -> Entity {
    let mut slider_input = builder.spawn(build_slider_input_node(
        root_components,
        start_value,
        build_button_bundle(Val::Px(200.0), Val::Px(12.0)),
    ));

    slider_input
        .with_children(|color_input| {
            color_input
                .spawn(build_slider_handle_node(start_value))
                .with_children(|slider_handle| {
                    slider_handle.spawn(build_slider_handle_bar_node(6.0));
                });
        })
        .id()
}

#[allow(unused)]
pub fn spawn_slider_input_with_image(
    builder: &mut ChildBuilder,
    root_components: impl Bundle,
    start_value: f32,
    image: Handle<Image>,
) -> Entity {
    let mut slider_input = builder.spawn(build_slider_input_node(
        root_components,
        start_value,
        build_button_bundle_with_image(Val::Px(200.0), Val::Px(12.0), image),
    ));

    slider_input
        .with_children(|color_input| {
            color_input
                .spawn(build_slider_handle_node(start_value))
                .with_children(|slider_handle| {
                    slider_handle.spawn(build_slider_handle_bar_node(6.0));
                });
        })
        .id()
}

fn send_slider_value_changed_events(
    mut on_changed: EventWriter<OnSliderInputValueChanged>,
    mut slider_query: Query<(
        &Interaction,
        &RelativeCursorPosition,
        &mut SliderInput,
        Entity,
    )>,
) {
    for (_, relative_cursor_position, mut slider_input, slider_entity) in slider_query
        .iter_mut()
        .filter(|slider| *slider.0 == Interaction::Pressed)
    {
        let Some(new_value) = calculate_slider_value(relative_cursor_position) else {
            continue;
        };

        if slider_input.value == new_value {
            continue;
        }

        slider_input.value = new_value;

        on_changed.send(OnSliderInputValueChanged::new(
            slider_entity,
            slider_input.value,
        ));
    }
}

fn update_slider_handle_position(
    mut on_changed: EventReader<OnSliderInputValueChanged>,
    mut slider_handle_query: Query<(Entity, &mut Style), With<SliderHandle>>,
    children_query: Query<&Children>,
) {
    for event in on_changed.read() {
        let (_, mut handle_style) = find_descendant_of_entity_mut(
            event.slider_input_entity(),
            &mut slider_handle_query,
            |item| item.0,
            &children_query,
        )
        .unwrap();

        handle_style.margin = calculate_handle_margin(event.new_value());
    }
}

fn calculate_handle_margin(slider_value: f32) -> UiRect {
    UiRect::left(Val::Percent(slider_value * 100.0))
}

fn calculate_slider_value(relative_cursor_position: &RelativeCursorPosition) -> Option<f32> {
    Some(relative_cursor_position.normalized?.x.clamp(0.0, 1.0))
}

// Node builders

fn build_slider_input_node(
    components: impl Bundle,
    start_value: f32,
    button_bundle: ButtonBundle,
) -> impl Bundle {
    (
        components,
        SliderInput::new(start_value),
        RelativeCursorPosition::default(),
        button_bundle,
    )
}

fn build_slider_handle_node(start_value: f32) -> impl Bundle {
    (
        SliderHandle,
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                margin: calculate_handle_margin(start_value),
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
