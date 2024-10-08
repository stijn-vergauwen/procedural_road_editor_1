use std::{
    fs::{read_dir, DirEntry},
    io,
};

use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    game_modes::GameMode,
    road::load::OnLoadRoadRequested,
    road_drawer::selected_road::OnSelectRoadRequested,
    ui::{
        components::buttons::{ButtonAction, OnButtonPressed},
        modal::{OnHideModalRequested, OnShowModalRequested},
    },
    GameRunningSet,
};

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (send_load_requests, show_modal_on_load_button_pressed)
                .in_set(GameRunningSet::SendCommands)
                .run_if(in_state(GameMode::RoadEditor).or_else(in_state(GameMode::RoadDrawer))),
        );
    }
}

#[derive(Component)]
struct RoadNameItem;

fn show_modal_on_load_button_pressed(
    mut on_pressed: EventReader<OnButtonPressed>,
    mut on_request: EventWriter<OnShowModalRequested>,
    mut commands: Commands,
) {
    for _ in on_pressed
        .read()
        .filter(|event| event.is_action(ButtonAction::LoadRoad))
    {
        let mut modal_content_container = commands.spawn(build_load_content_container_node());
        let modal_content_entity = modal_content_container.id();

        let road_names = get_saved_road_file_names().unwrap_or(Vec::new());

        modal_content_container.with_children(|container| {
            for name in road_names {
                container
                    .spawn(build_road_item_node())
                    .with_children(|road_item| {
                        road_item.spawn(build_road_item_text_node(&name, 24.0));
                    });
            }
        });

        on_request.send(OnShowModalRequested::new(modal_content_entity));
    }
}

fn send_load_requests(
    mut on_load_request: EventWriter<OnLoadRoadRequested>,
    mut on_select_request: EventWriter<OnSelectRoadRequested>,
    mut on_hide_request: EventWriter<OnHideModalRequested>,
    button_query: Query<(&Interaction, &Children), (With<RoadNameItem>, Changed<Interaction>)>,
    text_query: Query<&Text>,
    game_mode: Res<State<GameMode>>,
) {
    for (_, button_children) in button_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Pressed)
    {
        let text = text_query.get(button_children[0]).unwrap();
        let road_name = text.sections[0].value.clone();

        match game_mode.get() {
            GameMode::RoadEditor => {
                on_load_request.send(OnLoadRoadRequested::new(road_name));
            }
            GameMode::RoadDrawer => {
                on_select_request.send(OnSelectRoadRequested::new(road_name));
            }
            _ => (),
        }

        on_hide_request.send(OnHideModalRequested);
    }
}

fn get_saved_road_file_names() -> io::Result<Vec<String>> {
    let dir_entries = read_dir("./assets/roads")?;

    Ok(dir_entries
        .filter_map(|entry| file_name_from_dir_entry(entry))
        .collect())
}

fn file_name_from_dir_entry(entry: io::Result<DirEntry>) -> Option<String> {
    entry
        .ok()?
        .path()
        .file_stem()?
        .to_os_string()
        .into_string()
        .ok()
}

fn build_load_content_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
        ..default()
    }
}

fn build_road_item_node() -> impl Bundle {
    (
        RoadNameItem,
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(4.0), Val::Px(2.0)),
                ..default()
            },
            background_color: EMERALD_300.into(),
            ..default()
        },
    )
}

fn build_road_item_text_node(text: &str, font_size: f32) -> impl Bundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: String::from(text),
                style: TextStyle {
                    color: NEUTRAL_900.into(),
                    font_size,
                    ..default()
                },
            }],
            ..default()
        },
        ..default()
    }
}
