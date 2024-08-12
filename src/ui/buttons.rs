// TODO: move the save, load, and delete buttons to the code that actually uses them (right now I'm splitting it in horizontal layers instead of vertical sections)
mod reorder_button;

use bevy::{color::palettes::tailwind::*, prelude::*};
use reorder_button::send_reorder_button_pressed_events;
pub use reorder_button::{
    spawn_reorder_button, OnReorderButtonPressed, ReorderButton, ReorderDirection,
};

use crate::GameRunningSet;

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnReorderButtonPressed>().add_systems(
            Update,
            send_reorder_button_pressed_events.in_set(GameRunningSet::GetUserInput),
        );
    }
}

pub fn spawn_button_node(
    builder: &mut ChildBuilder,
    button_components: impl Bundle,
    text: &str,
    font_size: f32,
) -> Entity {
    let button_node = build_button_node(button_components);

    builder
        .spawn(button_node)
        .with_children(|button| {
            button.spawn(build_button_text_node(text, font_size));
        })
        .id()
}

fn build_button_node(button_components: impl Bundle) -> impl Bundle {
    (
        button_components,
        ButtonBundle {
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

fn build_button_text_node(text: &str, font_size: f32) -> impl Bundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: String::from(text),
                style: TextStyle {
                    color: NEUTRAL_900.into(),
                    font_size,
                    ..default()
                },
            }],
            ..default()
        },
        ..default()
    }
}
