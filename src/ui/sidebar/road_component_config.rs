use bevy::prelude::*;

use crate::{
    road::{ActiveRoad, OnRoadComponentChangeRequested},
    ui::{
        get_selected_road_component_index,
        inputs::{
            number_input::{spawn_number_input_node, OnNumberInputValueChanged},
            text_input::{spawn_text_input_node, OnTextInputValueChanged},
        },
        toolbar::components::{
            selected_road_component::{OnRoadComponentDeselected, OnRoadComponentSelected},
            RoadComponentItem,
        },
        ListItem,
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
    width_input_entity: Entity,
    height_input_entity: Entity,
    title_input_entity: Entity,
}

impl RoadComponentConfig {
    pub fn new(
        width_input_entity: Entity,
        height_input_entity: Entity,
        title_input_entity: Entity,
    ) -> Self {
        Self {
            width_input_entity,
            height_input_entity,
            title_input_entity,
        }
    }

    fn entity_matches_width_input(&self, entity: Entity) -> bool {
        self.width_input_entity == entity
    }

    fn entity_matches_height_input(&self, entity: Entity) -> bool {
        self.height_input_entity == entity
    }

    fn entity_matches_title_input(&self, entity: Entity) -> bool {
        self.title_input_entity == entity
    }
}

fn generate_config_section_for_selected_component(
    mut on_selected: EventReader<OnRoadComponentSelected>,
    mut commands: Commands,
    sidebar_query: Query<Entity, With<Sidebar>>,
) {
    for event in on_selected.read() {
        let sidebar = sidebar_query.single();
        let component_data = event.component_data();

        commands
            .entity(sidebar)
            .despawn_descendants()
            .with_children(|sidebar| {
                let mut component_config = sidebar.spawn(build_config_container_node());
                let mut width_input_entity = None;
                let mut height_input_entity = None;
                let mut title_input_entity = None;

                component_config.with_children(|container| {
                    title_input_entity =
                        Some(spawn_text_input_node(container, component_data.name()));

                    width_input_entity = Some(spawn_number_input_node(
                        container,
                        "Width",
                        component_data.size().x,
                        0.0..10.0,
                    ));

                    height_input_entity = Some(spawn_number_input_node(
                        container,
                        "Height",
                        component_data.size().y,
                        0.0..10.0,
                    ));
                });

                component_config.insert(RoadComponentConfig::new(
                    width_input_entity.unwrap(),
                    height_input_entity.unwrap(),
                    title_input_entity.unwrap(),
                ));
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
    component_item_query: Query<(&RoadComponentItem, &ListItem)>,
    active_road: Res<ActiveRoad>,
) {
    for event in on_input_changed.read() {
        let event_entity = event.number_input_entity();
        let component_config = component_config_query.single();

        let Some(selected_component_index) =
            get_selected_road_component_index(&component_item_query)
        else {
            continue;
        };

        let current_component_data =
            active_road.road_data().components()[selected_component_index].clone();
        let mut new_component_data = current_component_data.clone();

        if component_config.entity_matches_width_input(event_entity) {
            new_component_data.with_size(Vec2::new(
                event.new_value(),
                current_component_data.size().y,
            ));
        } else if component_config.entity_matches_height_input(event_entity) {
            new_component_data.with_size(Vec2::new(
                current_component_data.size().x,
                event.new_value(),
            ));
        }

        on_change_request.send(OnRoadComponentChangeRequested::new(
            selected_component_index,
            current_component_data,
            new_component_data,
        ));
    }
}

fn handle_text_input_changed_events(
    mut on_input_changed: EventReader<OnTextInputValueChanged>,
    mut on_change_request: EventWriter<OnRoadComponentChangeRequested>,
    component_config_query: Query<&RoadComponentConfig>,
    component_item_query: Query<(&RoadComponentItem, &ListItem)>,
    active_road: Res<ActiveRoad>,
) {
    for event in on_input_changed.read() {
        let event_entity = event.text_input_entity();
        let component_config = component_config_query.single();

        let Some(selected_component_index) =
            get_selected_road_component_index(&component_item_query)
        else {
            continue;
        };

        let current_component_data =
            active_road.road_data().components()[selected_component_index].clone();
        let mut new_component_data = current_component_data.clone();

        if component_config.entity_matches_title_input(event_entity) {
            new_component_data.with_name(event.text().into());
        }

        on_change_request.send(OnRoadComponentChangeRequested::new(
            selected_component_index,
            current_component_data,
            new_component_data,
        ));
    }
}

fn build_config_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}
