mod road_component_config;

use bevy::{color::palettes::tailwind::*, prelude::*};
use road_component_config::RoadComponentConfigPlugin;

pub struct SidebarPlugin;

impl Plugin for SidebarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoadComponentConfigPlugin)
            .add_systems(Startup, spawn_sidebar);
    }
}

// TODO: Show and hide sidebar depending on if a road component is selected (first allow a component to be 'selected')

#[derive(Component)]
pub struct Sidebar;

fn spawn_sidebar(mut commands: Commands) {
    commands
        .spawn(build_container_node())
        .with_children(|container| {
            container.spawn(build_sidebar_node());
        });
}

fn build_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }
}

fn build_sidebar_node() -> impl Bundle {
    (
        Sidebar,
        NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: BackgroundColor(CYAN_800.with_alpha(0.4).into()),
            ..default()
        },
    )
}
