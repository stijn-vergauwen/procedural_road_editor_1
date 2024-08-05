use bevy::{color::palettes::tailwind::*, prelude::*, ui::FocusPolicy};

use crate::GameRunningSet;

// TODO: listen to keypresses when selected
// TODO: update input text with keypresses
// TODO: when user presses enter, save new value
// TODO: when deselected, return text to value before selected

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnTextInputSelected>()
            .add_event::<OnTextInputDeselected>()
            .add_systems(
                Update,
                (
                    (
                        select_input_when_clicked,
                        deselect_input_on_click,
                        deselect_input_on_esc,
                    )
                        .in_set(GameRunningSet::GetUserInput),
                    (update_input_border_color).in_set(GameRunningSet::UpdateEntities),
                ),
            );
    }
}

#[derive(Component, Default)]
pub struct TextInput {
    is_selected: bool,
}

#[derive(Component)]
pub struct TextDisplay {
    input_entity: Entity,
}

#[derive(Event)]
pub struct OnTextInputSelected {
    text_input_entity: Entity,
}

impl OnTextInputSelected {
    pub fn new(text_input_entity: Entity) -> Self {
        Self { text_input_entity }
    }
}

#[derive(Event)]
pub struct OnTextInputDeselected {
    text_input_entity: Entity,
}

impl OnTextInputDeselected {
    pub fn new(text_input_entity: Entity) -> Self {
        Self { text_input_entity }
    }
}

pub fn spawn_text_input_node(builder: &mut ChildBuilder, text: impl Into<String>) -> Entity {
    let mut text_input = builder.spawn(build_text_input_node());
    let text_input_entity = text_input.id();

    text_input.with_children(|text_input| {
        text_input.spawn(build_text_display_node(text, text_input_entity));
    });

    text_input_entity
}

// Systems

fn select_input_when_clicked(
    mut on_selected: EventWriter<OnTextInputSelected>,
    mut input_query: Query<(&Interaction, &mut TextInput, Entity), Changed<Interaction>>,
) {
    for (_, mut input, entity) in input_query.iter_mut().filter(|(interaction, input, _)| {
        **interaction == Interaction::Pressed && !input.is_selected
    }) {
        input.is_selected = true;

        on_selected.send(OnTextInputSelected::new(entity));
    }
}

fn deselect_input_on_click(
    mut on_deselected: EventWriter<OnTextInputDeselected>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut input_query: Query<(&Interaction, &mut TextInput, Entity)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (_, mut input, entity) in input_query.iter_mut().filter(|(interaction, input, _)| {
            **interaction == Interaction::None && input.is_selected
        }) {
            input.is_selected = false;

            on_deselected.send(OnTextInputDeselected::new(entity));
        }
    }
}

fn deselect_input_on_esc(
    mut on_deselected: EventWriter<OnTextInputDeselected>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut input_query: Query<(&mut TextInput, Entity)>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        for (mut input, entity) in input_query
            .iter_mut()
            .filter(|(input, _)| input.is_selected)
        {
            input.is_selected = false;

            on_deselected.send(OnTextInputDeselected::new(entity));
        }
    }
}

fn update_input_border_color(
    mut on_selected: EventReader<OnTextInputSelected>,
    mut on_deselected: EventReader<OnTextInputDeselected>,
    mut input_query: Query<&mut BorderColor, (With<TextInput>, Changed<TextInput>)>,
) {
    for event in on_selected.read() {
        if let Ok(mut border_color) = input_query.get_mut(event.text_input_entity) {
            border_color.0 = CYAN_300.into();
        };
    }

    for event in on_deselected.read() {
        if let Ok(mut border_color) = input_query.get_mut(event.text_input_entity) {
            border_color.0 = NEUTRAL_900.into();
        };
    }
}

fn build_text_input_node() -> impl Bundle {
    (
        TextInput::default(),
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(4.0), Val::Px(2.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor(NEUTRAL_900.into()),
            focus_policy: FocusPolicy::Block,
            ..default()
        },
        Interaction::default(),
    )
}

fn build_text_display_node(text: impl Into<String>, input_entity: Entity) -> impl Bundle {
    (
        TextDisplay { input_entity },
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text.into(),
                    style: TextStyle {
                        color: Color::WHITE,
                        font_size: 30.0,
                        ..default()
                    },
                }],
                ..default()
            },
            ..default()
        },
    )
}
