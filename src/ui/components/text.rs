use bevy::{prelude::*, text::BreakLineOn};

use super::UiComponentBuilder;

pub struct TextConfig {
    pub text: String,
    pub color: Color,
    pub font_size: f32,
    pub justify: JustifyText,
    pub linebreak_behavior: BreakLineOn,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            text: String::new(),
            color: Color::WHITE,
            font_size: 24.0,
            justify: JustifyText::default(),
            linebreak_behavior: BreakLineOn::default(),
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

    pub fn default_with_text(text: impl Into<String>) -> Self {
        Self::new(TextConfig {
            text: text.into(),
            ..default()
        })
    }

    pub fn with_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.config.text = text.into();
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
