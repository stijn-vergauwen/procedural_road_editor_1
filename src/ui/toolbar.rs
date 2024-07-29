mod components;
mod save;

use bevy::{color::palettes::tailwind::*, prelude::*};
use components::ToolbarComponentsPlugin;
use save::SavePlugin;

use super::{
    buttons::{build_button_node, SaveButton},
    List,
};

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ToolbarComponentsPlugin, SavePlugin))
            .add_systems(Startup, spawn_toolbar);
    }
}

#[derive(Component)]
struct ToolBar;

#[derive(Component)]
struct RoadComponentsList;

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
        NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.0)),
                align_items: AlignItems::Stretch,
                column_gap: Val::Px(32.0),
                ..default()
            },
            background_color: BackgroundColor(CYAN_800.with_alpha(0.4).into()),
            ..default()
        },
    );

    let road_components_list_node = (
        RoadComponentsList,
        List,
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                column_gap: Val::Px(16.0),
                ..default()
            },
            ..default()
        },
    );

    commands.spawn(container_node).with_children(|container| {
        container.spawn(toolbar_node).with_children(|toolbar| {
            spawn_action_buttons(toolbar);

            toolbar.spawn(road_components_list_node);
        });
    });
}

fn spawn_action_buttons(builder: &mut ChildBuilder) {
    let action_buttons_container_node = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(4.0),
            ..default()
        },
        ..default()
    };

    builder
        .spawn(action_buttons_container_node)
        .with_children(|container| {
            build_button_node(container, SaveButton, "Save", 24.0);
        });
}
