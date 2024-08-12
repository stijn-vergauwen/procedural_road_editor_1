use bevy::prelude::*;

use crate::{
    road::{ActiveRoad, OnSaveRoadRequested},
    ui::{
        buttons::{spawn_button_node, ButtonAction, OnButtonPressed},
        inputs::text_input::{spawn_text_input_node, TextInput},
        modal::{OnHideModalRequested, OnShowModalRequested},
    },
    GameRunningSet,
};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (send_save_requests, show_modal_on_save_button_pressed)
                .in_set(GameRunningSet::SendCommands),
        );
    }
}

// TODO: replace with buttonAction
#[derive(Component)]
struct SaveConfirmButton;

#[derive(Component)]
struct RoadNameInput;

fn show_modal_on_save_button_pressed(
    mut on_pressed: EventReader<OnButtonPressed>,
    mut on_request: EventWriter<OnShowModalRequested>,
    mut commands: Commands,
) {
    for _ in on_pressed
        .read()
        .filter(|event| event.is_action(ButtonAction::SaveRoad))
    {
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
