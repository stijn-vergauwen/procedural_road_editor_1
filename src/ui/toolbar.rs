mod components;
mod load;
mod save;

use bevy::{color::palettes::tailwind::*, prelude::*};
use components::ToolbarComponentsPlugin;
use load::LoadPlugin;
use save::SavePlugin;

use super::{
    buttons::{spawn_button_node, LoadButton, SaveButton},
    List,
};

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ToolbarComponentsPlugin, SavePlugin, LoadPlugin))
            .add_systems(Startup, spawn_toolbar);
    }
}

#[derive(Component)]
struct ToolBar;

#[derive(Component)]
struct RoadComponentsList;

pub fn spawn_toolbar(mut commands: Commands) {
    commands
        .spawn(build_container_node())
        .with_children(|container| {
            container
                .spawn(build_toobar_node())
                .with_children(|toolbar| {
                    spawn_action_buttons(toolbar);

                    toolbar.spawn(build_road_components_list_node());
                });
        });
}

fn build_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Stretch,
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }
}

fn build_toobar_node() -> impl Bundle {
    (
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
    )
}

fn build_road_components_list_node() -> impl Bundle {
    (
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
    )
}

fn spawn_action_buttons(builder: &mut ChildBuilder) {
    builder
        .spawn(build_action_buttons_container_node())
        .with_children(|container| {
            spawn_button_node(container, SaveButton, "Save", 24.0);
            spawn_button_node(container, LoadButton, "Load", 24.0);
        });
}

fn build_action_buttons_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(4.0),
            ..default()
        },
        ..default()
    }
}
