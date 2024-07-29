mod components;

use bevy::{color::palettes::tailwind::*, prelude::*};
use components::ToolbarComponentsPlugin;

use super::List;

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToolbarComponentsPlugin)
            .add_systems(Startup, spawn_toolbar);
    }
}

#[derive(Component)]
struct ToolBar;

pub fn spawn_toolbar(mut commands: Commands) {
    let container_node = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Stretch,
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    };

    let toolbar_node = (
        ToolBar,
        List,
        NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                column_gap: Val::Px(16.0),
                ..default()
            },
            background_color: BackgroundColor(CYAN_800.with_alpha(0.4).into()),
            ..default()
        },
    );

    commands.spawn(container_node).with_children(|container| {
        container.spawn(toolbar_node);
    });
}
