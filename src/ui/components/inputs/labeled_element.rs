use bevy::prelude::*;

use crate::ui::components::{
    content_size::ContentSizeConfig,
    flexbox::FlexboxConfig,
    section::{SectionBuilder, SectionConfig},
    text::TextConfig,
    UiComponentWithChildrenBuilder,
};

#[derive(Clone)]
pub struct LabeledElementConfig {
    flexbox: FlexboxConfig,
    size: ContentSizeConfig,
    text: TextConfig,
}

impl LabeledElementConfig {
    pub fn top_label(text: impl Into<String>) -> Self {
        Self {
            flexbox: FlexboxConfig::column(),
            text: TextConfig::default().with_text(text).clone(),
            ..default()
        }
    }

    pub fn centered_top_label(text: impl Into<String>) -> Self {
        Self {
            flexbox: FlexboxConfig::horizontally_centered_column(),
            text: TextConfig::default().with_text(text).clone(),
            ..default()
        }
    }

    pub fn side_label(text: impl Into<String>) -> Self {
        Self {
            flexbox: FlexboxConfig::row(),
            text: TextConfig::default().with_text(text).clone(),
            ..default()
        }
    }

    pub fn with_content_size_config(
        mut self,
        content_size_config: ContentSizeConfig,
    ) -> Self {
        self.size = content_size_config;
        self
    }
}

impl Default for LabeledElementConfig {
    fn default() -> Self {
        Self {
            flexbox: FlexboxConfig::column(),
            size: ContentSizeConfig::full_width(),
            text: TextConfig::default(),
        }
    }
}

/// A UiComponent that adds a text label to it's child element.
#[derive(Default)]
pub struct LabeledElementBuilder {
    config: LabeledElementConfig,
}

impl LabeledElementBuilder {
    pub fn new(config: LabeledElementConfig) -> Self {
        Self { config }
    }

    pub fn top_label(text: impl Into<String>) -> Self {
        Self::new(LabeledElementConfig::top_label(text))
    }

    pub fn centered_top_label(text: impl Into<String>) -> Self {
        Self::new(LabeledElementConfig::centered_top_label(text))
    }

    pub fn side_label(text: impl Into<String>) -> Self {
        Self::new(LabeledElementConfig::side_label(text))
    }
}

impl UiComponentWithChildrenBuilder for LabeledElementBuilder {
    fn spawn(
        &self,
        builder: &mut ChildBuilder,
        components: impl Bundle,
        children: impl FnOnce(&mut ChildBuilder),
    ) -> Entity {
        SectionBuilder::new(
            SectionConfig::empty()
                .with_flexbox_config(self.config.flexbox)
                .with_content_size_config(self.config.size),
        )
        .spawn(builder, (), |flexbox| {
            flexbox.spawn((self.build(), components));

            children(flexbox);
        })
    }

    fn build(&self) -> impl Bundle {
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: self.config.text.text.clone(),
                    style: TextStyle {
                        color: self.config.text.color,
                        font_size: self.config.text.font_size,
                        ..default()
                    },
                }],
                justify: self.config.text.justify,
                linebreak_behavior: self.config.text.linebreak_behavior,
            },
            ..default()
        }
    }
}
