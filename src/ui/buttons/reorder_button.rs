use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::ui::ListItem;

// TODO: either remove these entity refs, or get them by iterating ancestors. Not yet sure which
#[derive(Event)]
pub struct OnReorderButtonPressed {
    list_entity: Entity,
    list_item_entity: Entity,
    direction: ReorderDirection,
}

impl OnReorderButtonPressed {
    pub fn new(list_entity: Entity, list_item_entity: Entity, direction: ReorderDirection) -> Self {
        Self {
            list_entity,
            list_item_entity,
            direction,
        }
    }

    pub fn list_entity(&self) -> Entity {
        self.list_entity
    }

    pub fn list_item_entity(&self) -> Entity {
        self.list_item_entity
    }

    pub fn direction(&self) -> ReorderDirection {
        self.direction
    }
}

#[derive(Component)]
pub struct ReorderButton {
    direction: ReorderDirection,
    // TODO: replace with iter_ancestors
    list_item_entity: Entity,
}

impl ReorderButton {
    pub fn direction(&self) -> ReorderDirection {
        self.direction
    }
    
    pub fn list_item_entity(&self) -> Entity {
        self.list_item_entity
    }
}

#[derive(Clone, Copy)]
pub enum ReorderDirection {
    Next,
    Previous,
}

pub fn send_reorder_button_pressed_events(
    mut on_pressed: EventWriter<OnReorderButtonPressed>,
    button_query: Query<(&ReorderButton, &Interaction), Changed<Interaction>>,
    list_item_query: Query<&ListItem>,
) {
    for (button, interaction) in button_query.iter() {
        if *interaction == Interaction::Pressed {
            let list_item = list_item_query.get(button.list_item_entity).unwrap();

            on_pressed.send(OnReorderButtonPressed::new(
                list_item.list_entity,
                button.list_item_entity,
                button.direction,
            ));
        }
    }
}

pub fn spawn_reorder_button(
    builder: &mut ChildBuilder,
    button_direction: ReorderDirection,
    list_item_entity: Entity,
    size: f32,
    visibility: Visibility,
) {
    builder
        .spawn(build_button_node(
            button_direction,
            list_item_entity,
            size,
            visibility,
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
    list_item_entity: Entity,
    size: f32,
    visibility: Visibility,
) -> (ReorderButton, ButtonBundle) {
    (
        ReorderButton {
            direction: button_direction,
            list_item_entity,
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
            visibility,
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
