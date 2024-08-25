use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::GameRunningSet;

use super::{
    content_wrap::{ContentWrapBuilder, ContentWrapConfig},
    text::{TextBuilder, TextConfig},
    UiComponentBuilder, UiComponentWithChildrenBuilder,
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

#[derive(Clone)]
pub struct ButtonConfig {
    pub background_image: Option<Handle<Image>>,
    pub wrap: ContentWrapConfig,
}

impl ButtonConfig {
    pub fn empty() -> Self {
        Self {
            background_image: None,
            wrap: ContentWrapConfig::empty(),
        }
    }

    pub fn with_background_image(&mut self, image_handle: Handle<Image>) -> &mut Self {
        self.background_image = Some(image_handle);
        self
    }

    pub fn with_content_wrap_config(&mut self, wrap: ContentWrapConfig) -> &mut Self {
        self.wrap = wrap;
        self
    }
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            background_image: None,
            wrap: ContentWrapConfig::wide_element()
                .with_background_color(NEUTRAL_500)
                .with_all_px_border_radius(8.0),
        }
    }
}

/// A button UiComponent without default content.
#[derive(Default)]
pub struct ButtonBuilder {
    config: ButtonConfig,
}

impl ButtonBuilder {
    pub fn new(config: ButtonConfig) -> Self {
        Self { config }
    }

    pub fn with_content_wrap_config(
        &mut self,
        content_wrap_config: ContentWrapConfig,
    ) -> &mut Self {
        self.config.wrap = content_wrap_config;
        self
    }

    pub fn with_background_color(
        &mut self,
        background_color: impl Into<BackgroundColor>,
    ) -> &mut Self {
        self.config.wrap.background_color = background_color.into();
        self
    }
}

impl UiComponentWithChildrenBuilder for ButtonBuilder {
    fn spawn(
        &self,
        builder: &mut ChildBuilder,
        components: impl Bundle,
        children: impl FnOnce(&mut ChildBuilder),
    ) -> Entity {
        ContentWrapBuilder::new(self.config.wrap).spawn(
            builder,
            (components, self.build()),
            children,
        )
    }

    fn build(&self) -> impl Bundle {
        match self.config.background_image.clone() {
            Some(image_handle) => (Button, Interaction::default(), UiImage::new(image_handle)),
            None => (Button, Interaction::default(), UiImage::default()),
        }
    }
}

// -- TextButton --

#[derive(Clone)]
pub struct TextButtonConfig {
    button: ButtonConfig,
    text: TextConfig,
}

impl TextButtonConfig {
    pub fn default_with_text(text: impl Into<String>) -> Self {
        let mut config = Self::default();
        config.text.with_text(text);
        config
    }
}

impl Default for TextButtonConfig {
    fn default() -> Self {
        Self {
            button: ButtonConfig::default(),
            text: TextConfig::default(),
        }
    }
}

/// A button UiComponent with text content.
#[derive(Default)]
pub struct TextButtonBuilder {
    config: TextButtonConfig,
}

impl TextButtonBuilder {
    pub fn new(config: TextButtonConfig) -> Self {
        Self { config }
    }

    pub fn default_with_text(text: impl Into<String>) -> Self {
        Self::new(TextButtonConfig::default_with_text(text))
    }

    pub fn with_button_config(&mut self, button_config: ButtonConfig) -> &mut Self {
        self.config.button = button_config;
        self
    }

    pub fn with_text_config(&mut self, text_config: TextConfig) -> &mut Self {
        self.config.text = text_config;
        self
    }

    pub fn with_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.config.text.with_text(text);
        self
    }
}

impl UiComponentBuilder for TextButtonBuilder {
    fn spawn(&self, builder: &mut ChildBuilder, components: impl Bundle) -> Entity {
        ButtonBuilder::new(self.config.button.clone()).spawn(builder, components, |button| {
            TextBuilder::new(self.config.text.clone()).spawn(button, ());
        })
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
