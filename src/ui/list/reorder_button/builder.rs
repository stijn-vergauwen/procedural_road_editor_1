use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::utility::partial::Partial;

use super::{ReorderButton, ReorderDirection};

pub fn spawn_reorder_button(
    builder: &mut ChildBuilder,
    button_direction: ReorderDirection,
    list_item_entity: Entity,
    size: f32,
) {
    builder
        .spawn(build_button_node(
            button_direction,
            list_item_entity,
            size,
        ))
        .with_children(|button| {
            button.spawn(build_button_text_node(button_direction, size));
        });
}

fn build_button_text_node(button_direction: ReorderDirection, size: f32) -> TextBundle {
    let button_text = get_reorder_button_text(button_direction);

    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: button_text,
                style: TextStyle {
                    color: NEUTRAL_900.into(),
                    font_size: size - 1.0,
                    ..default()
                },
            }],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    }
}

fn build_button_node(
    button_direction: ReorderDirection,
    main_entity: Entity,
    size: f32,
) -> impl Bundle {
    (
        Partial::new(main_entity),
        ReorderButton {
            direction: button_direction,
        },
        ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(size),
                height: Val::Px(size),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor(NEUTRAL_900.into()),
            ..default()
        },
    )
}

fn get_reorder_button_text(button_direction: ReorderDirection) -> String {
    match button_direction {
        ReorderDirection::Next => String::from(">"),
        ReorderDirection::Previous => String::from("<"),
    }
}
