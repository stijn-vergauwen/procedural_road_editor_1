use bevy::prelude::*;

use super::{
    content_size::ContentSizeConfig, content_wrap::ContentWrapConfig, flexbox::FlexboxConfig,
    UiComponentWithChildrenBuilder,
};

#[derive(Clone, Copy)]
pub struct SectionConfig {
    pub wrap: ContentWrapConfig,
    pub flexbox: FlexboxConfig,
    pub size: ContentSizeConfig,
}

impl SectionConfig {
    pub fn empty() -> Self {
        Self {
            wrap: ContentWrapConfig::empty(),
            flexbox: FlexboxConfig::default(),
            size: ContentSizeConfig::empty(),
        }
    }

    pub fn with_content_wrap_config(mut self, content_wrap_config: ContentWrapConfig) -> Self {
        self.wrap = content_wrap_config;
        self
    }

    pub fn with_flexbox_config(mut self, flexbox_config: FlexboxConfig) -> Self {
        self.flexbox = flexbox_config;
        self
    }

    pub fn with_content_size_config(mut self, content_size_config: ContentSizeConfig) -> Self {
        self.size = content_size_config;
        self
    }

    pub fn with_background_color(mut self, background_color: impl Into<BackgroundColor>) -> Self {
        self.wrap = self.wrap.with_background_color(background_color);
        self
    }
}

impl Default for SectionConfig {
    fn default() -> Self {
        Self {
            wrap: ContentWrapConfig::default()
                .with_all_px_padding(20.0)
                .with_all_px_border_radius(16.0),
            flexbox: FlexboxConfig::horizontally_centered_column().with_px_gap(12.0),
            size: ContentSizeConfig::default(),
        }
    }
}

/// A general section UiComponent for content, with content wrap, size, and layout.
#[derive(Default)]
pub struct SectionBuilder {
    config: SectionConfig,
}

impl SectionBuilder {
    pub fn new(config: SectionConfig) -> Self {
        Self { config }
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
                width: self.config.size.width,
                height: self.config.size.height,
                min_width: self.config.size.min_width,
                min_height: self.config.size.min_height,
                ..default()
            },
            background_color: self.config.wrap.background_color,
            border_color: self.config.wrap.border_color,
            border_radius: self.config.wrap.border_radius,
            focus_policy: self.config.wrap.focus_policy,
            ..default()
        }
    }
}
