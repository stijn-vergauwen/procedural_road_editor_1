use bevy::prelude::*;

use crate::{
    road::{
        active_road::{
            road_component_change::OnRoadComponentChangeRequested,
            road_component_deletion::OnRoadComponentDeletionRequested,
        },
        ActiveRoad,
    },
    ui::{
        buttons::{spawn_button_node, DeleteButton, OnDeleteButtonPressed},
        inputs::{
            color_input::{spawn_color_input, ColorInput, OnColorInputValueChanged},
            number_input::{spawn_number_input_node, NumberInput, OnNumberInputValueChanged},
            text_input::{spawn_text_input_node, OnTextInputValueChanged, TextInput},
        },
        toolbar::components::selected_road_component::{
            OnRoadComponentDeselected, OnRoadComponentSelected,
        },
    },
    GameRunningSet,
};

use super::Sidebar;

pub struct RoadComponentConfigPlugin;

impl Plugin for RoadComponentConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    handle_number_input_changed_events,
                    handle_text_input_changed_events,
                    handle_color_input_changed_events,
                    handle_delete_button_pressed_events,
                )
                    .in_set(GameRunningSet::SendCommands),
                (
                    generate_config_section_for_selected_component,
                    despawn_config_section_on_component_deselected,
                )
                    .in_set(GameRunningSet::DespawnEntities),
            ),
        );
    }
}

// TODO: handle on component deselected events, hide config. Wait this already exists, check where deselected events are sent <- doing

#[derive(Component)]
pub struct RoadComponentConfig {
    component_index: usize,
    component_entity: Entity,
}

impl RoadComponentConfig {
    pub fn new(component_index: usize, component_entity: Entity) -> Self {
        Self {
            component_index,
            component_entity,
        }
    }
}

#[derive(Clone, Copy, Component, PartialEq)]
enum ComponentConfigInputType {
    Name,
    Width,
    Height,
    Color,
}

fn generate_config_section_for_selected_component(
    mut on_selected: EventReader<OnRoadComponentSelected>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    sidebar_query: Query<Entity, With<Sidebar>>,
    active_road: Res<ActiveRoad>,
) {
    for event in on_selected.read() {
        let sidebar = sidebar_query.single();
        let component_index = event.component_index();
        let component_data = &active_road.road_data().components()[component_index];

        commands
            .entity(sidebar)
            .despawn_descendants()
            .with_children(|sidebar| {
                let mut component_config = sidebar.spawn(build_config_container_node(
                    component_index,
                    event.component_item_entity(),
                ));

                component_config.with_children(|container| {
                    spawn_text_input_node(
                        container,
                        ComponentConfigInputType::Name,
                        component_data.name(),
                    );

                    spawn_number_input_node(
                        container,
                        ComponentConfigInputType::Width,
                        "Width",
                        component_data.size().x,
                        0.0..10.0,
                    );

                    spawn_number_input_node(
                        container,
                        ComponentConfigInputType::Height,
                        "Height",
                        component_data.size().y,
                        0.0..10.0,
                    );

                    spawn_color_input(
                        container,
                        ComponentConfigInputType::Color,
                        &mut images,
                        component_data.color(),
                        Some("Color"),
                    );

                    spawn_button_node(container, DeleteButton, "Delete", 24.0);
                });
            });
    }
}

fn despawn_config_section_on_component_deselected(
    mut on_deselected: EventReader<OnRoadComponentDeselected>,
    mut commands: Commands,
    sidebar_query: Query<Entity, With<Sidebar>>,
) {
    for _ in on_deselected.read() {
        let sidebar = sidebar_query.single();

        commands.entity(sidebar).despawn_descendants();
    }
}

fn handle_number_input_changed_events(
    mut on_input_changed: EventReader<OnNumberInputValueChanged>,
    mut on_change_request: EventWriter<OnRoadComponentChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    number_input_query: Query<&ComponentConfigInputType, With<NumberInput>>,
    active_road: Res<ActiveRoad>,
) {
    for event in on_input_changed.read() {
        let event_entity = event.number_input_entity();
        let config_input_type = number_input_query.get(event_entity).unwrap();

        let component_config = component_config_query.single();
        let current_component_data =
            active_road.component_at_index(component_config.component_index);

        let new_size = match config_input_type {
            ComponentConfigInputType::Width => {
                Vec2::new(event.new_value(), current_component_data.size().y)
            }
            ComponentConfigInputType::Height => {
                Vec2::new(current_component_data.size().x, event.new_value())
            }
            _ => continue,
        };

        let requested_data = current_component_data.clone().with_size(new_size);

        on_change_request.send(OnRoadComponentChangeRequested::new(
            requested_data,
            component_config.component_entity,
            component_config.component_index,
        ));
    }
}

fn handle_text_input_changed_events(
    mut on_input_changed: EventReader<OnTextInputValueChanged>,
    mut on_change_request: EventWriter<OnRoadComponentChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    text_input_query: Query<&ComponentConfigInputType, With<TextInput>>,
    active_road: Res<ActiveRoad>,
) {
    for event in on_input_changed.read() {
        let event_entity = event.text_input_entity();
        let config_input_type = text_input_query.get(event_entity).unwrap();

        if *config_input_type != ComponentConfigInputType::Name {
            continue;
        };

        let component_config = component_config_query.single();
        let current_component_data =
            active_road.component_at_index(component_config.component_index);

        let requested_data = current_component_data
            .clone()
            .with_name(event.text().to_string());

        on_change_request.send(OnRoadComponentChangeRequested::new(
            requested_data,
            component_config.component_entity,
            component_config.component_index,
        ));
    }
}

fn handle_color_input_changed_events(
    mut on_input_changed: EventReader<OnColorInputValueChanged>,
    mut on_change_request: EventWriter<OnRoadComponentChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    color_input_query: Query<&ComponentConfigInputType, With<ColorInput>>,
    active_road: Res<ActiveRoad>,
) {
    for event in on_input_changed.read() {
        let event_entity = event.color_input_entity();
        let config_input_type = color_input_query.get(event_entity).unwrap();

        if *config_input_type != ComponentConfigInputType::Color {
            continue;
        };

        let component_config = component_config_query.single();
        let current_component_data =
            active_road.component_at_index(component_config.component_index);

        let requested_data = current_component_data.clone().with_color(event.new_color());

        on_change_request.send(OnRoadComponentChangeRequested::new(
            requested_data,
            component_config.component_entity,
            component_config.component_index,
        ));
    }
}

fn handle_delete_button_pressed_events(
    mut on_button_pressed: EventReader<OnDeleteButtonPressed>,
    mut on_deletion_request: EventWriter<OnRoadComponentDeletionRequested>,
    component_config_query: Query<&RoadComponentConfig>,
) {
    for _ in on_button_pressed.read() {
        let component_config = component_config_query.single();

        on_deletion_request.send(OnRoadComponentDeletionRequested::new(
            component_config.component_entity,
            component_config.component_index,
        ));
    }
}

fn build_config_container_node(component_index: usize, component_entity: Entity) -> impl Bundle {
    (
        RoadComponentConfig::new(component_index, component_entity),
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
    )
}
