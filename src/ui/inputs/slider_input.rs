use bevy::{color::palettes::tailwind::*, prelude::*, ui::RelativeCursorPosition};

use crate::GameRunningSet;

pub struct SliderInputPlugin;

impl Plugin for SliderInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSliderInputValueChanged>()
            .add_systems(
                Update,
                (
                    update_slider_value.in_set(GameRunningSet::UpdateEntities),
                    print_slider_events,
                ),
            );
    }
}

#[derive(Component)]
struct SliderInput {
    value: f32,
}

impl SliderInput {
    fn new(value: f32) -> Self {
        Self { value }
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
pub fn spawn_slider_input(builder: &mut ChildBuilder, root_components: impl Bundle) -> Entity {
    let mut slider_input = builder.spawn(build_slider_input_node(
        Val::Px(200.0),
        Val::Px(12.0),
        root_components,
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

fn update_slider_value(
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
        let Some(relative_position) = relative_cursor_position.normalized else {
            continue;
        };

        let Some((_, mut handle_style)) = slider_handle_query
            .iter_mut()
            .find(|(slider_handle, _)| slider_handle.slider_input_entity == slider_entity)
        else {
            continue;
        };

        let new_value = relative_position.x.clamp(0.0, 1.0);

        if new_value == slider_input.value {
            continue;
        }

        slider_input.value = new_value;
        handle_style.margin = UiRect::left(Val::Percent(slider_input.value * 100.0));

        on_changed.send(OnSliderInputValueChanged::new(
            slider_entity,
            slider_input.value,
        ));
    }
}

fn print_slider_events(mut events: EventReader<OnSliderInputValueChanged>) {
    for event in events.read() {
        println!("Slider value: {}", event.new_value());
    }
}

// Node builders

fn build_slider_input_node(width: Val, height: Val, components: impl Bundle) -> impl Bundle {
    (
        components,
        SliderInput::new(0.0),
        RelativeCursorPosition::default(),
        ButtonBundle {
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