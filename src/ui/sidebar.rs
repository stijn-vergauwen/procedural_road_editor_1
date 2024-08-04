mod road_component_config;

use bevy::{color::palettes::tailwind::*, prelude::*};
use road_component_config::RoadComponentConfigPlugin;

use crate::GameRunningSet;

pub struct SidebarPlugin;

impl Plugin for SidebarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoadComponentConfigPlugin)
            .add_systems(Startup, spawn_sidebar)
            .add_systems(
                Update,
                toggle_sidebar_visibility_based_on_content.after(GameRunningSet::DespawnEntities),
            );
    }
}

#[derive(Component)]
pub struct Sidebar;

fn spawn_sidebar(mut commands: Commands) {
    commands
        .spawn(build_container_node())
        .with_children(|container| {
            container.spawn(build_sidebar_node());
        });
}

fn toggle_sidebar_visibility_based_on_content(
    mut sidebar_query: Query<(Option<&Children>, &mut Style), With<Sidebar>>,
) {
    let (children, mut style) = sidebar_query.single_mut();

    let current_display = style.display;
    let target_display = match children {
        Some(_) => Display::Flex,
        None => Display::None,
    };

    if current_display != target_display {
        style.display = target_display;
    }
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
