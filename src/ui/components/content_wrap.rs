use bevy::{color::palettes::tailwind::*, prelude::*};

use super::UiComponentWithChildrenBuilder;

#[derive(Clone, Copy)]
pub struct ContentWrapConfig {
    pub padding: UiRect,
    pub background_color: BackgroundColor,
    pub border_size: UiRect,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl ContentWrapConfig {
    // TODO: "wide_element" preset for buttons
    pub fn wide_element() -> Self {
        Self {
            padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
            ..default()
        }
    }

    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_all_px_padding(self, padding: f32) -> Self {
        self.with_padding(UiRect::all(Val::Px(padding)))
    }

    pub fn with_background_color(mut self, background_color: impl Into<BackgroundColor>) -> Self {
        self.background_color = background_color.into();
        self
    }

    pub fn with_border_radius(mut self, border_radius: BorderRadius) -> Self {
        self.border_radius = border_radius;
        self
    }

    pub fn with_all_px_border_radius(self, border_radius: f32) -> Self {
        self.with_border_radius(BorderRadius::all(Val::Px(border_radius)))
    }

    pub fn squared(self) -> Self {
        self.with_border_radius(BorderRadius::ZERO)
    }

    pub fn rounded(self) -> Self {
        self.with_border_radius(BorderRadius::MAX)
    }
}

impl Default for ContentWrapConfig {
    fn default() -> Self {
        Self {
            padding: UiRect::all(Val::Px(12.0)),
            background_color: NEUTRAL_700.into(),
            border_size: UiRect::ZERO,
            border_color: Color::NONE.into(),
            border_radius: BorderRadius::ZERO,
        }
    }
}

/// A UiComponent to wrap color and space around it's content.
#[derive(Default)]
pub struct ContentWrapBuilder {
    config: ContentWrapConfig,
}

impl ContentWrapBuilder {
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

impl UiComponentWithChildrenBuilder for ContentWrapBuilder {
    fn build(&self) -> impl Bundle {
        NodeBundle {
            style: Style {
                padding: self.config.padding,
                border: self.config.border_size,
                ..default()
            },
            background_color: self.config.background_color,
            border_color: self.config.border_color,
            border_radius: self.config.border_radius,
            ..default()
        }
    }
}
