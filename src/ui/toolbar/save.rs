use bevy::prelude::*;

use crate::{
    road::{ActiveRoad, OnSaveRoadRequested},
    ui::{
        buttons::spawn_button_node,
        inputs::text_input::{spawn_text_input_node, TextInput},
        modal::{OnHideModalRequested, OnShowModalRequested},
    },
    GameRunningSet,
};

use super::ToolbarAction;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSaveButtonPressed>().add_systems(
            Update,
            (
                send_save_button_pressed_events.in_set(GameRunningSet::GetUserInput),
                (send_save_requests, show_modal_on_save_button_pressed)
                    .in_set(GameRunningSet::SendCommands),
            ),
        );
    }
}

#[derive(Component)]
struct SaveConfirmButton;

#[derive(Component)]
struct RoadNameInput;

#[derive(Event)]
struct OnSaveButtonPressed;

// TODO: replace with generic system
fn send_save_button_pressed_events(
    mut on_pressed: EventWriter<OnSaveButtonPressed>,
    button_query: Query<(&Interaction, &ToolbarAction), Changed<Interaction>>,
) {
    for (interaction, action) in button_query.iter() {
        if *interaction == Interaction::Pressed && *action == ToolbarAction::SaveRoad {
            on_pressed.send(OnSaveButtonPressed);
        }
    }
}

fn show_modal_on_save_button_pressed(
    mut events: EventReader<OnSaveButtonPressed>,
    mut on_request: EventWriter<OnShowModalRequested>,
    mut commands: Commands,
) {
    for _ in events.read() {
        let mut modal_content_container = commands.spawn(build_save_content_container_node());
        let modal_content_entity = modal_content_container.id();

        modal_content_container.with_children(|container| {
            spawn_text_input_node(container, RoadNameInput, "New road");

            spawn_button_node(container, SaveConfirmButton, "Confirm", 24.0);
        });

        on_request.send(OnShowModalRequested::new(modal_content_entity));
    }
}

fn send_save_requests(
    mut on_save_request: EventWriter<OnSaveRoadRequested>,
    mut on_hide_request: EventWriter<OnHideModalRequested>,
    active_road: Res<ActiveRoad>,
    road_name_input_query: Query<&TextInput, With<RoadNameInput>>,
    button_query: Query<&Interaction, (With<SaveConfirmButton>, Changed<Interaction>)>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            let road_name_input = road_name_input_query.single();

            let mut road_data = active_road.road_data().clone();
            road_data.set_name(road_name_input.current_text().into());

            on_save_request.send(OnSaveRoadRequested::new(road_data));

            on_hide_request.send(OnHideModalRequested);
        }
    }
}

fn build_save_content_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
            ..default()
        },
        ..default()
    }
}
