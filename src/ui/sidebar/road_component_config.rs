use bevy::prelude::*;

use crate::{
    road::{
        active_road::{
            road_component_change::OnRoadComponentChangeRequested,
            road_component_deletion::OnRoadComponentDeletionRequested,
        },
        ActiveRoad, RoadComponent,
    },
    ui::{
        components::{
            buttons::{ButtonAction, OnButtonPressed, TextButtonBuilder},
            inputs::{
                color_input::{
                    ColorInput, ColorInputBuilder, ColorInputConfig, OnColorInputValueChanged,
                },
                number_input::{NumberInput, NumberInputBuilder, OnNumberInputValueChanged},
            },
            UiComponentBuilder,
        },
        inputs::text_input::{spawn_text_input_node, OnTextInputValueChanged, TextInput},
        list::{List, ListItem},
        toolbar::components::selected_road_component::{
            OnRoadComponentDeselected, OnRoadComponentSelected,
        },
    },
    utility::find_ancestor_of_entity,
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

#[derive(Component)]
pub struct RoadComponentConfig {
    /// Reference to the [RoadComponentItem] that is currently being configured.
    component_entity: Entity,
}

impl RoadComponentConfig {
    pub fn new(component_entity: Entity) -> Self {
        Self { component_entity }
    }
}

#[derive(Clone, Copy, Component, PartialEq)]
enum ComponentConfigAction {
    SetName,
    SetWidth,
    SetHeight,
    SetColor,
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
        let component_data = active_road.component_at_index(component_index);

        commands
            .entity(sidebar)
            .despawn_descendants()
            .with_children(|sidebar| {
                let mut component_config =
                    sidebar.spawn(build_config_container_node(event.component_item_entity()));

                component_config.with_children(|container| {
                    spawn_text_input_node(
                        container,
                        ComponentConfigAction::SetName,
                        component_data.name(),
                    );

                    // TODO: add "Width" label
                    NumberInputBuilder::default()
                        .with_values(component_data.size().x, 0.0..10.0)
                        .spawn(container, ComponentConfigAction::SetWidth);

                    // TODO: add "Height" label
                    NumberInputBuilder::default()
                        .with_values(component_data.size().y, 0.0..10.0)
                        .spawn(container, ComponentConfigAction::SetHeight);

                    // TODO: add "Color" label
                    ColorInputBuilder::new(
                        ColorInputConfig::default().with_start_color(component_data.color()),
                        &mut images,
                    )
                    .spawn(container, ComponentConfigAction::SetColor);

                    TextButtonBuilder::default_with_text("Delete")
                        .spawn(container, ButtonAction::DeleteComponent);
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
    number_input_query: Query<&ComponentConfigAction, With<NumberInput>>,
    list_item_query: Query<&ListItem>,
) {
    for event in on_input_changed.read() {
        let event_entity = event.number_input_entity();
        let config_action = number_input_query.get(event_entity).unwrap();

        let component_config = component_config_query.single();
        let new_value = event.new_value();

        let requested_change: Box<dyn Fn(RoadComponent) -> RoadComponent + Send + Sync> =
            match config_action {
                ComponentConfigAction::SetWidth => {
                    Box::new(move |road_component: RoadComponent| -> RoadComponent {
                        road_component.with_x(new_value)
                    })
                }
                ComponentConfigAction::SetHeight => {
                    Box::new(move |road_component: RoadComponent| -> RoadComponent {
                        road_component.with_y(new_value)
                    })
                }
                _ => continue,
            };

        on_change_request.send(OnRoadComponentChangeRequested::new(
            requested_change,
            component_config.component_entity,
            list_item_index_from_entity(&list_item_query, component_config.component_entity),
        ));
    }
}

fn handle_text_input_changed_events(
    mut on_input_changed: EventReader<OnTextInputValueChanged>,
    mut on_change_request: EventWriter<OnRoadComponentChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    text_input_query: Query<&ComponentConfigAction, With<TextInput>>,
    list_item_query: Query<&ListItem>,
) {
    for event in on_input_changed.read() {
        let event_entity = event.text_input_entity();
        let config_action = text_input_query.get(event_entity).unwrap();

        if *config_action != ComponentConfigAction::SetName {
            continue;
        };

        let component_config = component_config_query.single();

        let name = event.text().to_string();

        on_change_request.send(OnRoadComponentChangeRequested::new(
            Box::new(move |road_component| road_component.with_name(name.clone())),
            component_config.component_entity,
            list_item_index_from_entity(&list_item_query, component_config.component_entity),
        ));
    }
}

fn handle_color_input_changed_events(
    mut on_input_changed: EventReader<OnColorInputValueChanged>,
    mut on_change_request: EventWriter<OnRoadComponentChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    color_input_query: Query<&ComponentConfigAction, With<ColorInput>>,
    list_item_query: Query<&ListItem>,
) {
    for event in on_input_changed.read() {
        let event_entity = event.color_input_entity();
        let config_action = color_input_query.get(event_entity).unwrap();

        if *config_action != ComponentConfigAction::SetColor {
            continue;
        };

        let component_config = component_config_query.single();
        let color = event.new_color();

        on_change_request.send(OnRoadComponentChangeRequested::new(
            Box::new(move |road_component| road_component.with_color(color)),
            component_config.component_entity,
            list_item_index_from_entity(&list_item_query, component_config.component_entity),
        ));
    }
}

fn handle_delete_button_pressed_events(
    mut on_pressed: EventReader<OnButtonPressed>,
    mut on_deletion_request: EventWriter<OnRoadComponentDeletionRequested>,
    mut on_deselect: EventWriter<OnRoadComponentDeselected>,
    component_config_query: Query<&RoadComponentConfig>,
    list_item_query: Query<(Entity, &ListItem)>,
    list_query: Query<Entity, With<List>>,
    parent_query: Query<&Parent>,
) {
    for _ in on_pressed
        .read()
        .filter(|event| event.is_action(ButtonAction::DeleteComponent))
    {
        let component_config = component_config_query.single();
        let (list_item_entity, list_item) = list_item_query
            .get(component_config.component_entity)
            .unwrap();

        let list_entity =
            find_ancestor_of_entity(list_item_entity, &list_query, |item| *item, &parent_query)
                .unwrap();

        on_deletion_request.send(OnRoadComponentDeletionRequested::new(
            list_entity,
            component_config.component_entity,
            list_item.index(),
        ));

        on_deselect.send(OnRoadComponentDeselected);
    }
}

fn list_item_index_from_entity(list_item_query: &Query<&ListItem>, entity: Entity) -> usize {
    list_item_query.get(entity).unwrap().index()
}

fn build_config_container_node(component_entity: Entity) -> impl Bundle {
    (
        RoadComponentConfig::new(component_entity),
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
