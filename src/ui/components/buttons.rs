use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::GameRunningSet;

use super::{
    content_wrap::{ContentWrap, ContentWrapConfig},
    UiComponentWithChildren,
};

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnButtonPressed>().add_systems(
            Update,
            send_button_pressed_events.in_set(GameRunningSet::GetUserInput),
        );
    }
}

// Start of new UiComponent code

// TODO: make ButtonsConfig struct with presets (like FlexboxConfig has)

/// A button UiComponent with text as content.
#[derive(Default)]
pub struct Button {
    config: ContentWrapConfig,
}

impl Button {
    pub fn new(config: ContentWrapConfig) -> Self {
        Self { config }
    }

    pub fn with_padding(&mut self, padding: UiRect) -> &mut Self {
        self.config.padding = padding;
        self
    }

    pub fn with_background_color(
        &mut self,
        background_color: impl Into<BackgroundColor>,
    ) -> &mut Self {
        self.config.background_color = background_color.into();
        self
    }

    pub fn with_border_size(&mut self, border_size: UiRect) -> &mut Self {
        self.config.border_size = border_size;
        self
    }

    pub fn with_border_color(&mut self, border_color: impl Into<BorderColor>) -> &mut Self {
        self.config.border_color = border_color.into();
        self
    }

    pub fn with_border_radius(&mut self, border_radius: BorderRadius) -> &mut Self {
        self.config.border_radius = border_radius;
        self
    }
}

impl UiComponentWithChildren for Button {
    fn spawn(
        &self,
        builder: &mut ChildBuilder,
        components: impl Bundle,
        children: impl FnOnce(&mut ChildBuilder),
    ) -> Entity {
        ContentWrap::new(self.config).spawn(builder, (components, self.build()), children)
    }

    fn build(&self) -> impl Bundle {
        (Button, Interaction::default())
    }
}

// End of new UiComponent code

#[derive(Event)]
pub struct OnButtonPressed {
    action: ButtonAction,
}

impl OnButtonPressed {
    pub fn new(action: ButtonAction) -> Self {
        Self { action }
    }

    pub fn is_action(&self, action: ButtonAction) -> bool {
        self.action == action
    }
}

#[derive(Component, PartialEq, Clone, Copy)]
pub enum ButtonAction {
    SaveRoad,
    LoadRoad,
    AddComponent,
    DeleteComponent,
}

fn send_button_pressed_events(
    mut on_pressed: EventWriter<OnButtonPressed>,
    button_query: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
) {
    for (interaction, action) in button_query.iter() {
        if *interaction == Interaction::Pressed {
            on_pressed.send(OnButtonPressed::new(*action));
        }
    }
}

pub fn spawn_button_node(
    builder: &mut ChildBuilder,
    root_components: impl Bundle,
    text: &str,
    font_size: f32,
) -> Entity {
    let button_node = build_button_node(root_components);

    builder
        .spawn(button_node)
        .with_children(|button| {
            button.spawn(build_button_text_node(text, font_size));
        })
        .id()
}

fn build_button_node(root_components: impl Bundle) -> impl Bundle {
    (
        root_components,
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
