pub mod components;
mod inputs;
pub mod list;
pub mod main_menu;
mod modal;
pub mod sidebar;
pub mod toolbar;

use bevy::prelude::*;
use components::{
    flexbox::{FlexboxBuilder, FlexboxConfig},
    UiComponentWithChildrenBuilder, UiComponentsPlugin,
};
use inputs::UiInputsPlugin;
use list::ListPlugin;
use main_menu::MainMenuPlugin;
use modal::ModalPlugin;
use sidebar::{spawn_sidebar, SidebarPlugin};
use toolbar::{spawn_toolbar, ToolbarPlugin};

use crate::{game_modes::GameMode, utility::despawn_component_recursive};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiComponentsPlugin,
            ToolbarPlugin,
            SidebarPlugin,
            UiInputsPlugin,
            ModalPlugin,
            ListPlugin,
            MainMenuPlugin,
        ))
        .add_systems(OnEnter(GameMode::RoadEditor), spawn_editor_layout)
        .add_systems(
            OnExit(GameMode::RoadEditor),
            despawn_component_recursive::<EditorLayoutContainer>,
        );
    }
}

// TODO: add LayoutContainer marker component, remove recursive when leaving editor mode

/// Marker component for the editor layout.
#[derive(Component)]
pub struct EditorLayoutContainer;

fn spawn_editor_layout(mut commands: Commands) {
    let container_node = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            row_gap: Val::Px(20.0),
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    };

    commands
        .spawn((container_node, EditorLayoutContainer))
        .with_children(|container| {
            FlexboxBuilder::new(FlexboxConfig::row().with_justify(JustifyContent::Start)).spawn(
                container,
                (),
                |left_side| {
                    spawn_sidebar(left_side);
                },
            );

            spawn_toolbar(container);
        });
}
