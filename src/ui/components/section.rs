use bevy::prelude::*;

use super::{
    content_wrap::ContentWrapConfig, flexbox::FlexboxConfig, UiComponentWithChildrenBuilder,
};

pub struct SectionConfig {
    pub wrap: ContentWrapConfig,
    pub flexbox: FlexboxConfig,
}

impl Default for SectionConfig {
    fn default() -> Self {
        Self {
            wrap: ContentWrapConfig::default()
                .with_all_px_padding(20.0)
                .with_all_px_border_radius(12.0),
            flexbox: FlexboxConfig::horizontally_centered_column().with_px_gap(12.0),
        }
    }
}

/// A general section UiComponent for content, with content wrap and layout.
#[derive(Default)]
pub struct SectionBuilder {
    config: SectionConfig,
}

impl SectionBuilder {
    pub fn new(config: SectionConfig) -> Self {
        Self { config }
    }

    pub fn with_content_wrap_config(
        &mut self,
        content_wrap_config: ContentWrapConfig,
    ) -> &mut Self {
        self.config.wrap = content_wrap_config;
        self
    }

    pub fn with_flexbox_config(&mut self, flexbox_config: FlexboxConfig) -> &mut Self {
        self.config.flexbox = flexbox_config;
        self
    }
}

impl UiComponentWithChildrenBuilder for SectionBuilder {
    fn build(&self) -> impl Bundle {
        NodeBundle {
            style: Style {
                flex_direction: self.config.flexbox.flex_direction,
                justify_content: self.config.flexbox.justify_content,
                align_items: self.config.flexbox.align_items,
                flex_wrap: self.config.flexbox.flex_wrap,
                row_gap: self.config.flexbox.row_gap,
                column_gap: self.config.flexbox.column_gap,
                padding: self.config.wrap.padding,
                border: self.config.wrap.border_size,
                ..default()
            },
            background_color: self.config.wrap.background_color,
            border_color: self.config.wrap.border_color,
            border_radius: self.config.wrap.border_radius,
            ..default()
        }
    }
}
