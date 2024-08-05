use bevy::{color::palettes::tailwind::*, prelude::*, ui::FocusPolicy};

// TODO: enable is_selected when user clicks input
// TODO: disable is_selected when user clicks outside of input
// TODO: disable is_selected on esc
// TODO: listen to keypresses when selected
// TODO: update input text with keypresses
// TODO: when user presses enter, save new value
// TODO: when deselected, return text to value before selected

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, ());
    }
}

#[derive(Component)]
pub struct TextInput;

#[derive(Component)]
pub struct TextDisplay {
    input_entity: Entity,
}

pub fn spawn_text_input_node(builder: &mut ChildBuilder, text: impl Into<String>) -> Entity {
    let mut text_input = builder.spawn(build_text_input_node());
    let text_input_entity = text_input.id();

    text_input.with_children(|text_input| {
        text_input.spawn(build_text_display_node(text, text_input_entity));
    });

    text_input_entity
}

fn build_text_input_node() -> impl Bundle {
    (
        TextInput,
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
