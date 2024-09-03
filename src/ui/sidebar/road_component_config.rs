use bevy::prelude::*;

use crate::{
    road::{
        active_road::active_road_events::{
            road_component_change::{RoadComponentChange, RoadComponentFieldChange},
            road_component_deletion::RoadComponentDeletion,
            ActiveRoadChange, OnActiveRoadChangeRequested, RoadComponentField,
        },
        ActiveRoad,
    },
    ui::{
        components::{
            buttons::{ButtonAction, OnButtonPressed, TextButtonBuilder},
            flexbox::{FlexboxBuilder, FlexboxConfig},
            inputs::{
                color_input::{
                    ColorInput, ColorInputBuilder, ColorInputConfig, OnColorInputValueChanged,
                },
                number_input::{NumberInput, NumberInputBuilder, OnNumberInputValueChanged},
            },
            UiComponentBuilder, UiComponentWithChildrenBuilder,
        },
        inputs::text_input::{spawn_text_input_node, OnTextInputValueChanged, TextInput},
        list::ListItem,
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
                FlexboxBuilder::new(
                    FlexboxConfig::horizontally_centered_column().with_px_gap(12.0),
                )
                .spawn(
                    sidebar,
                    RoadComponentConfig::new(event.component_item_entity()),
                    |config_container| {
                        // TODO: replace with text input UiComponent
                        spawn_text_input_node(
                            config_container,
                            RoadComponentField::Name,
                            component_data.name.clone(),
                        );

                        // TODO: add "Width" label
                        NumberInputBuilder::default()
                            .with_values(component_data.size.x, 0.0..10.0)
                            .spawn(config_container, RoadComponentField::Width);

                        // TODO: add "Height" label
                        NumberInputBuilder::default()
                            .with_values(component_data.size.y, 0.0..10.0)
                            .spawn(config_container, RoadComponentField::Height);

                        // TODO: add "Color" label
                        ColorInputBuilder::new(
                            ColorInputConfig::default().with_start_color(component_data.color),
                            &mut images,
                        )
                        .spawn(config_container, RoadComponentField::Color);

                        TextButtonBuilder::default_with_text("Delete")
                            .spawn(config_container, ButtonAction::DeleteComponent);
                    },
                );
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
    mut on_change_request: EventWriter<OnActiveRoadChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    number_input_query: Query<&RoadComponentField, With<NumberInput>>,
    list_item_query: Query<&ListItem>,
) {
    for event in on_input_changed.read() {
        let Ok(config_action) = number_input_query.get(event.number_input_entity()) else {
            continue;
        };

        let component_config = component_config_query.single();
        let new_value = event.new_value();

        let field_to_change = match config_action {
            RoadComponentField::Width => RoadComponentFieldChange::Width(new_value),
            RoadComponentField::Height => RoadComponentFieldChange::Height(new_value),
            _ => continue,
        };

        on_change_request.send(OnActiveRoadChangeRequested::new(
            ActiveRoadChange::ChangeRoadComponent(RoadComponentChange::new(
                field_to_change,
                list_item_index_from_entity(&list_item_query, component_config.component_entity),
            )),
        ));
    }
}

fn handle_text_input_changed_events(
    mut on_input_changed: EventReader<OnTextInputValueChanged>,
    mut on_change_request: EventWriter<OnActiveRoadChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    text_input_query: Query<&RoadComponentField, With<TextInput>>,
    list_item_query: Query<&ListItem>,
) {
    for event in on_input_changed.read() {
        let Ok(config_action) = text_input_query.get(event.text_input_entity()) else {
            continue;
        };

        if *config_action != RoadComponentField::Name {
            continue;
        };

        let component_config = component_config_query.single();

        let name = event.text().to_string();

        on_change_request.send(OnActiveRoadChangeRequested::new(
            ActiveRoadChange::ChangeRoadComponent(RoadComponentChange::new(
                RoadComponentFieldChange::Name(name),
                list_item_index_from_entity(&list_item_query, component_config.component_entity),
            )),
        ));
    }
}

fn handle_color_input_changed_events(
    mut on_input_changed: EventReader<OnColorInputValueChanged>,
    mut on_change_request: EventWriter<OnActiveRoadChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    color_input_query: Query<&RoadComponentField, With<ColorInput>>,
    list_item_query: Query<&ListItem>,
) {
    for event in on_input_changed.read() {
        let Ok(config_action) = color_input_query.get(event.color_input_entity()) else {
            continue;
        };

        if *config_action != RoadComponentField::Color {
            continue;
        };

        let component_config = component_config_query.single();
        let color = event.new_color();

        on_change_request.send(OnActiveRoadChangeRequested::new(
            ActiveRoadChange::ChangeRoadComponent(RoadComponentChange::new(
                RoadComponentFieldChange::Color(color),
                list_item_index_from_entity(&list_item_query, component_config.component_entity),
            )),
        ));
    }
}

fn handle_delete_button_pressed_events(
    mut on_pressed: EventReader<OnButtonPressed>,
    mut on_deletion_request: EventWriter<OnActiveRoadChangeRequested>,
    mut on_deselect: EventWriter<OnRoadComponentDeselected>,
    component_config_query: Query<&RoadComponentConfig>,
    list_item_query: Query<&ListItem>,
) {
    for _ in on_pressed
        .read()
        .filter(|event| event.is_action(ButtonAction::DeleteComponent))
    {
        let component_config = component_config_query.single();
        let list_item = list_item_query
            .get(component_config.component_entity)
            .unwrap();

        on_deletion_request.send(OnActiveRoadChangeRequested::new(
            ActiveRoadChange::DeleteRoadComponent(RoadComponentDeletion::new(list_item.index())),
        ));

        on_deselect.send(OnRoadComponentDeselected);
    }
}

fn list_item_index_from_entity(list_item_query: &Query<&ListItem>, entity: Entity) -> usize {
    list_item_query.get(entity).unwrap().index()
}
