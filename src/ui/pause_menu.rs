use bevy::prelude::*;

use crate::{game_modes::GameMode, GameRunningSet};

use super::{
    components::{
        buttons::TextButtonBuilder,
        flexbox::{FlexboxBuilder, FlexboxConfig},
        text::TextBuilder,
        UiComponentBuilder, UiComponentWithChildrenBuilder,
    },
    modal::{OnHideModalRequested, OnShowModalRequested},
};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (show_pause_menu_on_esc, handle_pause_menu_actions)
                .in_set(GameRunningSet::GetUserInput)
                .run_if(in_state(GameMode::RoadEditor).or_else(in_state(GameMode::RoadDrawer))),
        );
    }
}

// todo: make show and hide events

#[derive(Component, Clone, Copy)]
enum PauseMenuAction {
    ExitToMainMenu,
}

fn show_pause_menu_on_esc(
    mut on_show_modal: EventWriter<OnShowModalRequested>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let flexbox_node =
            FlexboxBuilder::new(FlexboxConfig::horizontally_centered_column().with_px_gap(16.0))
                .build();

        let modal_content = commands
            .spawn(flexbox_node)
            .with_children(|container| {
                TextBuilder::default()
                    .with_text("Pause menu")
                    .spawn(container, ());

                TextButtonBuilder::default_with_text("Exit to main menu")
                    .spawn(container, PauseMenuAction::ExitToMainMenu);
            })
            .id();

        on_show_modal.send(OnShowModalRequested::new(modal_content));
    }
}

fn handle_pause_menu_actions(
    mut on_hide_modal: EventWriter<OnHideModalRequested>,
    mut next_game_mode: ResMut<NextState<GameMode>>,
    button_query: Query<(&Interaction, &PauseMenuAction), Changed<Interaction>>,
) {
    for (_, action) in button_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Pressed)
    {
        match action {
            PauseMenuAction::ExitToMainMenu => next_game_mode.set(GameMode::MainMenu),
        }

        on_hide_modal.send(OnHideModalRequested);
    }
}
