use bevy::prelude::*;

use super::UiComponentWithChildrenBuilder;

#[derive(Clone, Copy)]
pub struct ContentSizeConfig {
    /// The width of this component, use only percentage values for consistency.
    pub width: Val,
    /// The height of this component, use only percentage values for consistency.
    pub height: Val,
    /// The minimum width this component has to be, use only pixel values for consistency.
    pub min_width: Val,
    /// The minimum height this component has to be, use only pixel values for consistency.
    pub min_height: Val,
}

impl ContentSizeConfig {
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a config with width of 100%
    pub fn full_width() -> Self {
        Self {
            width: Val::Percent(100.0),
            ..default()
        }
    }

    /// Creates a config with width & height of 100%
    pub fn full() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        }
    }

    /// Returns this component but with the given width in percentages.
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Val::Percent(width);
        self
    }

    /// Returns this component but with a width of 100%.
    pub fn with_full_width(self) -> Self {
        self.with_width(100.0)
    }

    /// Returns this component but with the given height in percentages.
    #[expect(unused)]
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Val::Percent(height);
        self
    }

    /// Returns this component but with the given minimum width in pixels.
    pub fn with_min_width(mut self, min_width: f32) -> Self {
        self.min_width = Val::Px(min_width);
        self
    }

    /// Returns this component but with the given minimum height in pixels.
    pub fn with_min_height(mut self, min_height: f32) -> Self {
        self.min_height = Val::Px(min_height);
        self
    }
}

impl Default for ContentSizeConfig {
    fn default() -> Self {
        Self {
            width: Val::Auto,
            height: Val::Auto,
            min_width: Val::Auto,
            min_height: Val::Auto,
        }
    }
}

/// A UiComponent to give a desired width & height to an element.
#[derive(Default)]
pub struct ContentSizeBuilder {
    config: ContentSizeConfig,
}

impl UiComponentWithChildrenBuilder for ContentSizeBuilder {
    fn build(&self) -> impl Bundle {
        NodeBundle {
            style: Style {
                width: self.config.width,
                height: self.config.height,
                min_width: self.config.min_width,
                min_height: self.config.min_height,
                ..default()
            },
            ..default()
        }
    }
}
