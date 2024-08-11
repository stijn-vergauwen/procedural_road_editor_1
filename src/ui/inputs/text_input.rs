use bevy::{
    color::palettes::tailwind::*,
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
    ui::FocusPolicy,
};

use crate::GameRunningSet;

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnTextInputSelected>()
            .add_event::<OnTextInputDeselected>()
            .add_event::<OnTextInputDisplayTextChanged>()
            .add_event::<OnTextInputValueChanged>()
            .add_systems(
                Update,
                (
                    (
                        select_input_when_clicked,
                        deselect_input_on_click,
                        deselect_input_on_esc,
                        handle_keyboard_input,
                        confirm_text_input_on_enter,
                    )
                        .in_set(GameRunningSet::GetUserInput),
                    (update_input_border_color, update_input_display_text)
                        .in_set(GameRunningSet::UpdateEntities),
                ),
            );
    }
}

#[derive(Component, Default)]
pub struct TextInput {
    is_selected: bool,
    current_text: String,
    text_being_edited: String,
}

impl TextInput {
    pub fn new(text: String) -> Self {
        Self {
            is_selected: false,
            current_text: text,
            text_being_edited: String::default(),
        }
    }

    pub fn select(&mut self) {
        self.is_selected = true;

        self.text_being_edited = self.current_text.clone();
    }

    pub fn cancel_edit(
        &mut self,
        on_changed: &mut EventWriter<OnTextInputDisplayTextChanged>,
        text_input_entity: Entity,
    ) {
        self.is_selected = false;
        self.text_being_edited.clear();

        on_changed.send(OnTextInputDisplayTextChanged::new(
            text_input_entity,
            self.current_text.clone(),
        ));
    }

    pub fn confirm_edit(
        &mut self,
        on_changed: &mut EventWriter<OnTextInputValueChanged>,
        text_input_entity: Entity,
    ) {
        self.is_selected = false;
        self.current_text = self.text_being_edited.clone();

        on_changed.send(OnTextInputValueChanged::new(
            text_input_entity,
            self.current_text.clone(),
        ));
    }

    pub fn update_text_being_edited(
        &mut self,
        keyboard_input: &Key,
        on_changed: &mut EventWriter<OnTextInputDisplayTextChanged>,
        text_input_entity: Entity,
    ) {
        self.text_being_edited =
            parse_keyboard_input(self.text_being_edited.clone(), keyboard_input);

        on_changed.send(OnTextInputDisplayTextChanged::new(
            text_input_entity,
            self.text_being_edited.clone(),
        ));
    }

    pub fn current_text(&self) -> &str {
        &self.current_text
    }
}

#[derive(Component)]
pub struct TextDisplay {
    // TODO: replace with iter_ancestors
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

#[derive(Event)]
pub struct OnTextInputDisplayTextChanged {
    text_input_entity: Entity,
    text: String,
}

impl OnTextInputDisplayTextChanged {
    pub fn new(text_input_entity: Entity, text: String) -> Self {
        Self {
            text_input_entity,
            text,
        }
    }
}

#[derive(Event)]
pub struct OnTextInputValueChanged {
    text_input_entity: Entity,
    text: String,
}

impl OnTextInputValueChanged {
    pub fn new(text_input_entity: Entity, text: String) -> Self {
        Self {
            text_input_entity,
            text,
        }
    }

    pub fn text_input_entity(&self) -> Entity {
        self.text_input_entity
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

pub fn spawn_text_input_node(
    builder: &mut ChildBuilder,
    extra_input_components: impl Bundle,
    text: impl Into<String>,
) -> Entity {
    let text = text.into();
    let mut text_input = builder.spawn(build_text_input_node(extra_input_components, text.clone()));
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
        input.select();

        on_selected.send(OnTextInputSelected::new(entity));
    }
}

fn deselect_input_on_click(
    mut on_deselected: EventWriter<OnTextInputDeselected>,
    mut on_changed: EventWriter<OnTextInputDisplayTextChanged>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut input_query: Query<(&Interaction, &mut TextInput, Entity)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (_, mut input, entity) in input_query.iter_mut().filter(|(interaction, input, _)| {
            **interaction == Interaction::None && input.is_selected
        }) {
            input.cancel_edit(&mut on_changed, entity);

            on_deselected.send(OnTextInputDeselected::new(entity));
        }
    }
}

fn deselect_input_on_esc(
    mut on_deselected: EventWriter<OnTextInputDeselected>,
    mut on_changed: EventWriter<OnTextInputDisplayTextChanged>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut input_query: Query<(&mut TextInput, Entity)>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        for (mut input, entity) in input_query
            .iter_mut()
            .filter(|(input, _)| input.is_selected)
        {
            input.cancel_edit(&mut on_changed, entity);

            on_deselected.send(OnTextInputDeselected::new(entity));
        }
    }
}

fn handle_keyboard_input(
    mut on_keyboard_input: EventReader<KeyboardInput>,
    mut on_changed: EventWriter<OnTextInputDisplayTextChanged>,
    mut input_query: Query<(&mut TextInput, Entity)>,
) {
    for event in on_keyboard_input
        .read()
        .filter(|event| event.state == ButtonState::Pressed)
    {
        for (mut input, text_input_entity) in input_query
            .iter_mut()
            .filter(|(input, _)| input.is_selected)
        {
            input.update_text_being_edited(&event.logical_key, &mut on_changed, text_input_entity);
        }
    }
}

fn confirm_text_input_on_enter(
    mut on_deselected: EventWriter<OnTextInputDeselected>,
    mut on_changed: EventWriter<OnTextInputValueChanged>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut input_query: Query<(&mut TextInput, Entity)>,
) {
    if key_input.just_pressed(KeyCode::Enter) {
        for (mut input, entity) in input_query
            .iter_mut()
            .filter(|(input, _)| input.is_selected)
        {
            input.confirm_edit(&mut on_changed, entity);

            on_deselected.send(OnTextInputDeselected::new(entity));
        }
    }
}

fn update_input_display_text(
    mut on_changed: EventReader<OnTextInputDisplayTextChanged>,
    mut text_display_query: Query<(&TextDisplay, &mut Text)>,
) {
    for event in on_changed.read() {
        let (_, mut text) = text_display_query.iter_mut()
        .find(|(text_display, _)| text_display.input_entity == event.text_input_entity)
        .expect("TextInputDisplayTextChanged event should always match TextInput entity with a display node.");

        text.sections[0].value = event.text.clone();
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

fn build_text_input_node(extra_components: impl Bundle, text: String) -> impl Bundle {
    (
        extra_components,
        TextInput::new(text),
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

fn build_text_display_node(text: String, input_entity: Entity) -> impl Bundle {
    (
        TextDisplay { input_entity },
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
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

fn parse_keyboard_input(mut text: String, keyboard_input: &Key) -> String {
    match keyboard_input {
        Key::Character(character) => {
            text.push_str(&character);
        }
        Key::Space => {
            text.push(' ');
        }
        Key::Backspace => {
            text.pop();
        }
        _ => (),
    }

    text
}
