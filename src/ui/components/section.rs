use bevy::prelude::*;

use super::{
    content_wrap::ContentWrapConfig, flexbox::FlexboxConfig, UiComponentWithChildrenBuilder,
};

#[derive(Clone, Copy)]
pub struct SectionConfig {
    pub wrap: ContentWrapConfig,
    pub flexbox: FlexboxConfig,
}

impl SectionConfig {
    pub fn empty() -> Self {
        Self {
            wrap: ContentWrapConfig::empty(),
            flexbox: FlexboxConfig::default(),
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

    /// Returns this component but with the given width
    /// 
    /// - Use only percentage values for consistency.
    pub fn with_width(mut self, width: Val) -> Self {
        self.wrap = self.wrap.with_width(width);
        self
    }

    /// Returns this component but with the width set to 100%.
    pub fn with_full_width(mut self) -> Self {
        self.wrap = self.wrap.with_full_width();
        self
    }

    /// Returns this component but with the given minimum width
    /// 
    /// - Use only pixel values for consistency.
    pub fn with_min_width(mut self, min_width: Val) -> Self {
        self.wrap = self.wrap.with_min_width(min_width);
        self
    }

    /// Returns this component but with the given height
    /// 
    /// - Use only percentage values for consistency.
    pub fn with_height(mut self, height: Val) -> Self {
        self.wrap = self.wrap.with_height(height);
        self
    }

    pub fn with_background_color(mut self, background_color: impl Into<BackgroundColor>) -> Self {
        self.wrap = self.wrap.with_background_color(background_color);
        self
    }

    pub fn squared(mut self) -> Self {
        self.wrap = self.wrap.with_border_radius(BorderRadius::ZERO);
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
                width: self.config.wrap.width,
                min_width: self.config.wrap.min_width,
                height: self.config.wrap.height,
                ..default()
            },
            background_color: self.config.wrap.background_color,
            border_color: self.config.wrap.border_color,
            border_radius: self.config.wrap.border_radius,
            ..default()
        }
    }
}
