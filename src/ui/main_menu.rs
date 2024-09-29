use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{game_modes::GameMode, utility::despawn_component_recursive};

use super::components::{
    buttons::TextButtonBuilder,
    content_size::ContentSizeConfig,
    content_wrap::ContentWrapConfig,
    flexbox::FlexboxConfig,
    section::{SectionBuilder, SectionConfig},
    UiComponentBuilder, UiComponentWithChildrenBuilder,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMode::MainMenu), spawn_main_menu)
            .add_systems(
                OnExit(GameMode::MainMenu),
                despawn_component_recursive::<MainMenu>,
            )
            .add_systems(
                Update,
                handle_state_transitions.run_if(in_state(GameMode::MainMenu)),
            );
    }
}

/// Marker component for the main menu.
#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MainMenuAction {
    LoadEditorMode,
}

fn spawn_main_menu(mut commands: Commands) {
    let section_node = SectionBuilder::new(SectionConfig {
        flexbox: FlexboxConfig::centered(),
        size: ContentSizeConfig::full(),
        wrap: ContentWrapConfig::empty().with_background_color(CYAN_300),
    })
    .build();

    commands
        .spawn((section_node, MainMenu))
        .with_children(|container| {
            SectionBuilder::default().spawn(container, (), |section| {
                TextButtonBuilder::default_with_text("Load editor")
                    .spawn(section, MainMenuAction::LoadEditorMode);
            });
        });
}

fn handle_state_transitions(
    button_query: Query<(&Interaction, &MainMenuAction), Changed<Interaction>>,
    mut next_game_mode: ResMut<NextState<GameMode>>,
) {
    for (_, action) in button_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Pressed)
    {
        match action {
            MainMenuAction::LoadEditorMode => next_game_mode.set(GameMode::RoadEditor),
        }
    }
}
