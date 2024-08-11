mod reorder;
pub mod selected_road_component;

use bevy::{color::palettes::tailwind::*, prelude::*, text::BreakLineOn};
use reorder::ReorderPlugin;
use selected_road_component::{OnRoadComponentSelected, SelectedRoadComponentPlugin};

use crate::{
    road::{
        active_road::{
            new_road_component::OnRoadComponentAdded,
            road_component_change::OnRoadComponentChanged,
            road_component_deletion::OnRoadComponentDeleted, OnActiveRoadSet,
        },
        RoadComponent,
    },
    ui::{
        buttons::{spawn_reorder_button, ReorderDirection},
        ListItem,
    },
    GameRunningSet,
};

use super::RoadComponentsList;

const COMPONENT_DISPLAY_SCALE: f32 = 50.0;

pub struct ToolbarComponentsPlugin;

impl Plugin for ToolbarComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ReorderPlugin, SelectedRoadComponentPlugin))
            .add_systems(
                Update,
                (
                    (handle_road_component_added, handle_road_component_changed)
                        .in_set(GameRunningSet::UpdateEntities),
                    (handle_active_road_set, handle_road_component_deleted)
                        .in_set(GameRunningSet::DespawnEntities),
                ),
            );
    }
}

#[derive(Component, Default)]
pub struct RoadComponentItem {
    is_selected: bool,
}

impl RoadComponentItem {
    pub fn new(is_selected: bool) -> Self {
        Self { is_selected }
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }
}

#[derive(Component)]
struct RoadComponentName;

#[derive(Component)]
struct RoadComponentDisplay;

fn handle_active_road_set(
    mut on_road_set: EventReader<OnActiveRoadSet>,
    mut commands: Commands,
    components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for event in on_road_set.read() {
        let components_list_entity = components_list_query.single();
        let road_components = event.road_data().components();
        let component_count = road_components.len();

        commands
            .entity(components_list_entity)
            .despawn_descendants()
            .with_children(|components_list| {
                for (index, road_component) in road_components.iter().enumerate() {
                    spawn_road_component(
                        components_list,
                        index,
                        components_list_entity,
                        road_component,
                        component_count,
                    );
                }
            });
    }
}

fn handle_road_component_added(
    mut on_added: EventReader<OnRoadComponentAdded>,
    mut on_component_selected: EventWriter<OnRoadComponentSelected>,
    mut commands: Commands,
    components_list_query: Query<Entity, With<RoadComponentsList>>,
) {
    for event in on_added.read() {
        let components_list_entity = components_list_query.single();

        // TODO: refactor out this entity
        let mut component_item_entity = None;

        commands
            .entity(components_list_entity)
            .with_children(|components_list| {
                component_item_entity = Some(spawn_road_component(
                    components_list,
                    event.component_index(),
                    components_list_entity,
                    event.component_data(),
                    event.component_count(),
                ));
            });

        // TODO: refactor out this if let
        if let Some(component_item_entity) = component_item_entity {
            on_component_selected.send(OnRoadComponentSelected::new(
                event.component_data().clone(),
                component_item_entity,
            ));
        }
    }
}

fn handle_road_component_changed(
    mut on_changed: EventReader<OnRoadComponentChanged>,
    component_item_query: Query<(Entity, &ListItem), With<RoadComponentItem>>,
    mut component_display_query: Query<
        (Entity, &mut Style, &mut BackgroundColor),
        With<RoadComponentDisplay>,
    >,
    mut component_name_query: Query<(Entity, &mut Text), With<RoadComponentName>>,
    parent_query: Query<&Parent>,
) {
    for event in on_changed.read() {
        // TODO: get component item entity from the event
        let Some((component_item_entity, _)) = component_item_query
            .iter()
            .find(|(_, list_item)| list_item.index() == event.component_index())
        else {
            continue;
        };

        let road_component = event.component_data();

        if let Some((_, mut style, mut background_color)) =
            component_display_query
                .iter_mut()
                .find(|(display_entity, _, _)| {
                    // TODO: split this parent query search to a utility function (will be reused a LOT :D)
                    parent_query
                        .iter_ancestors(*display_entity)
                        .find(|ancestor| *ancestor == component_item_entity)
                        .is_some()
                })
        {
            *style = build_component_display_style(road_component);
            *background_color = road_component.color().into();
        }

        if let Some((_, mut text)) = component_name_query.iter_mut().find(|(display_entity, _)| {
            // TODO: split this parent query search to a utility function (will be reused a LOT :D)
            parent_query
                .iter_ancestors(*display_entity)
                .find(|ancestor| *ancestor == component_item_entity)
                .is_some()
        }) {
            text.sections[0].value = road_component.name().to_string();
        }
    }
}

// TODO: read reorder events

fn handle_road_component_deleted(
    mut on_deleted: EventReader<OnRoadComponentDeleted>,
    mut commands: Commands,
    component_item_query: Query<(Entity, &ListItem), With<RoadComponentItem>>,
) {
    for event in on_deleted.read() {
        // TODO: get component item entity from the event
        let Some((component_item_entity, _)) = component_item_query
            .iter()
            .find(|(_, list_item)| list_item.index() == event.component_index())
        else {
            continue;
        };

        let mut component_item_commands = commands.entity(component_item_entity);
        component_item_commands.remove_parent();
        component_item_commands.despawn_recursive();
    }
}

// Utility

fn spawn_road_component(
    components_list: &mut ChildBuilder,
    index: usize,
    components_list_entity: Entity,
    road_component: &RoadComponent,
    component_count: usize,
) -> Entity {
    let mut container = components_list.spawn(build_road_components_container_node(ListItem::new(
        components_list_entity,
        index as u8,
    )));
    let container_entity = container.id();

    container.with_children(|container| {
        container.spawn(build_component_display_node(road_component));
        container.spawn(build_component_text_node(road_component));

        container
            .spawn(build_button_container_node())
            .with_children(|button_container| {
                if index > 0 {
                    spawn_reorder_button(
                        button_container,
                        ReorderDirection::Previous,
                        container_entity,
                        26.0,
                    );
                }

                if index < component_count - 1 {
                    spawn_reorder_button(
                        button_container,
                        ReorderDirection::Next,
                        container_entity,
                        26.0,
                    );
                }
            });
    });

    container_entity
}

fn build_road_components_container_node(list_item: ListItem) -> impl Bundle {
    (
        list_item,
        RoadComponentItem::default(),
        Interaction::default(),
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                ..default()
            },
            ..default()
        },
    )
}

fn build_component_display_node(road_component: &RoadComponent) -> impl Bundle {
    (
        RoadComponentDisplay,
        NodeBundle {
            style: build_component_display_style(road_component),
            background_color: road_component.color().into(),
            ..default()
        },
    )
}

fn build_component_display_style(road_component: &RoadComponent) -> Style {
    Style {
        width: Val::Px(road_component.size().x * COMPONENT_DISPLAY_SCALE),
        height: Val::Px(road_component.size().y * COMPONENT_DISPLAY_SCALE),
        ..default()
    }
}

fn build_component_text_node(road_component: &RoadComponent) -> impl Bundle {
    (
        RoadComponentName,
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: road_component.name().to_string(),
                    style: TextStyle {
                        color: NEUTRAL_900.into(),
                        ..default()
                    },
                }],
                linebreak_behavior: BreakLineOn::NoWrap,
                justify: JustifyText::Center,
            },
            ..default()
        },
    )
}

fn build_button_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            column_gap: Val::Px(8.0),
            ..default()
        },
        ..default()
    }
}
