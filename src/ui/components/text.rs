use bevy::{prelude::*, text::BreakLineOn};

use super::UiComponentBuilder;

#[derive(Clone)]
pub struct TextConfig {
    pub text: String,
    pub color: Color,
    pub font_size: f32,
    pub justify: JustifyText,
    pub linebreak_behavior: BreakLineOn,
}

impl TextConfig {
    pub fn with_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }

    pub fn with_justify(&mut self, justify: JustifyText) -> &mut Self {
        self.justify = justify;
        self
    }
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            text: String::new(),
            color: Color::WHITE,
            font_size: 24.0,
            justify: JustifyText::Left,
            linebreak_behavior: BreakLineOn::WordBoundary,
        }
    }
}

/// A simple single-line text UiComponent.
#[derive(Default)]
pub struct TextBuilder {
    config: TextConfig,
}

impl TextBuilder {
    pub fn new(config: TextConfig) -> Self {
        Self { config }
    }

    pub fn with_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.config.with_text(text);
        self
    }
}

impl UiComponentBuilder for TextBuilder {
    fn build(&self) -> impl Bundle {
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: self.config.text.clone(),
                    style: TextStyle {
                        color: self.config.color,
                        font_size: self.config.font_size,
                        ..default()
                    },
                }],
                justify: self.config.justify,
                linebreak_behavior: self.config.linebreak_behavior,
            },
            ..default()
        }
    }
}
