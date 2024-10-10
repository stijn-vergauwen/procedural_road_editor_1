use bevy::prelude::*;

use super::UiComponentWithChildrenBuilder;

#[derive(Clone, Copy)]
pub struct FlexboxConfig {
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub flex_wrap: FlexWrap,
    pub row_gap: Val,
    pub column_gap: Val,
}

impl FlexboxConfig {
    pub fn row() -> Self {
        Self::default()
    }

    pub fn horizontally_centered_row() -> Self {
        Self {
            justify_content: JustifyContent::Center,
            ..default()
        }
    }

    #[expect(unused)]
    pub fn vertically_centered_row() -> Self {
        Self {
            align_items: AlignItems::Center,
            ..default()
        }
    }

    pub fn column() -> Self {
        Self {
            flex_direction: FlexDirection::Column,
            ..default()
        }
    }

    pub fn horizontally_centered_column() -> Self {
        Self {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    #[expect(unused)]
    pub fn vertically_centered_column() -> Self {
        Self {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }

    pub fn centered() -> Self {
        Self {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    pub fn with_justify(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    pub fn with_align(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }

    #[expect(unused)]
    pub fn with_row_gap(mut self, gap: Val) -> Self {
        self.row_gap = gap;
        self
    }

    #[expect(unused)]
    pub fn with_column_gap(mut self, gap: Val) -> Self {
        self.column_gap = gap;
        self
    }

    pub fn with_gap(mut self, gap: Val) -> Self {
        self.row_gap = gap;
        self.column_gap = gap;
        self
    }

    pub fn with_px_gap(self, gap: f32) -> Self {
        self.with_gap(Val::Px(gap))
    }
}

impl Default for FlexboxConfig {
    fn default() -> Self {
        Self {
            flex_direction: Default::default(),
            justify_content: Default::default(),
            align_items: Default::default(),
            flex_wrap: Default::default(),
            row_gap: Val::ZERO,
            column_gap: Val::ZERO,
        }
    }
}

/// A flexbox UiComponent, controls layout without padding or background
#[derive(Default)]
pub struct FlexboxBuilder {
    config: FlexboxConfig,
}

impl FlexboxBuilder {
    pub fn new(config: FlexboxConfig) -> Self {
        Self { config }
    }
}

impl UiComponentWithChildrenBuilder for FlexboxBuilder {
    fn build(&self) -> impl Bundle {
        let conf = &self.config;

        NodeBundle {
            style: Style {
                flex_direction: conf.flex_direction,
                justify_content: conf.justify_content,
                align_items: conf.align_items,
                flex_wrap: conf.flex_wrap,
                row_gap: conf.row_gap,
                column_gap: conf.column_gap,
                ..default()
            },
            ..default()
        }
    }
}
