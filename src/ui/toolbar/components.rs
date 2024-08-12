mod reorder;
pub mod selected_road_component;

use bevy::{color::palettes::tailwind::*, prelude::*, text::BreakLineOn};
use reorder::ReorderPlugin;
use selected_road_component::{
    OnRoadComponentDeselected, OnRoadComponentSelected, SelectedRoadComponentPlugin,
};

use crate::{
    road::{
        active_road::{
            new_road_component::OnRoadComponentAdded,
            road_component_change::OnRoadComponentChanged,
            road_component_deletion::OnRoadComponentDeleted,
            road_component_reorder::OnRoadComponentReordered, OnActiveRoadSet,
        },
        ActiveRoad, RoadComponent,
    },
    ui::{
        buttons::{spawn_reorder_button, ReorderButton, ReorderDirection},
        ListItem,
    },
    utility::entity_is_descendant_of,
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
                    (
                        (
                            add_road_component_on_event,
                            update_reorder_buttons_on_add_event,
                        )
                            .chain(),
                        update_road_component_on_change,
                        (
                            reorder_road_components_on_event,
                            update_reorder_buttons_on_reorder_event,
                        )
                            .chain(),
                    )
                        .in_set(GameRunningSet::UpdateEntities),
                    (
                        rebuild_road_components_on_active_road_set,
                        delete_road_component_on_event,
                    )
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

fn rebuild_road_components_on_active_road_set(
    mut on_road_set: EventReader<OnActiveRoadSet>,
    mut on_deselect: EventWriter<OnRoadComponentDeselected>,
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

        on_deselect.send(OnRoadComponentDeselected);
    }
}

fn add_road_component_on_event(
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
                event.component_index(),
                component_item_entity,
            ));
        }
    }
}

fn update_reorder_buttons_on_add_event(
    mut on_added: EventReader<OnRoadComponentAdded>,
    components_list_query: Query<Entity, With<RoadComponentsList>>,
    mut reorder_button_query: Query<(Entity, &ReorderButton, &mut Visibility)>,
    list_item_query: Query<&ListItem>,
    parent_query: Query<&Parent>,
    active_road: Res<ActiveRoad>,
) {
    for _ in on_added.read() {
        let component_list_entity = components_list_query.single();
        let component_count = active_road.component_count();

        update_visibility_of_component_reorder_buttons(
            &mut reorder_button_query,
            &parent_query,
            &list_item_query,
            component_list_entity,
            component_count,
        );
    }
}

fn update_road_component_on_change(
    mut on_changed: EventReader<OnRoadComponentChanged>,
    mut component_display_query: Query<
        (Entity, &mut Style, &mut BackgroundColor),
        With<RoadComponentDisplay>,
    >,
    mut component_name_query: Query<(Entity, &mut Text), With<RoadComponentName>>,
    parent_query: Query<&Parent>,
) {
    for event in on_changed.read() {
        let component_entity = event.component_entity();
        let road_component = event.component_data();

        if let Some((_, mut style, mut background_color)) =
            component_display_query
                .iter_mut()
                .find(|(display_entity, _, _)| {
                    entity_is_descendant_of(&parent_query, *display_entity, component_entity)
                })
        {
            update_component_display(&mut style, &mut background_color, road_component);
        }

        if let Some((_, mut text)) = component_name_query.iter_mut().find(|(name_entity, _)| {
            entity_is_descendant_of(&parent_query, *name_entity, component_entity)
        }) {
            update_component_name(&mut text, road_component);
        }
    }
}

fn reorder_road_components_on_event(
    mut on_reordered: EventReader<OnRoadComponentReordered>,
    mut components_list_query: Query<&mut Children, With<RoadComponentsList>>,
    mut component_item_query: Query<&mut ListItem, With<RoadComponentItem>>,
) {
    for event in on_reordered.read() {
        let mut component_list_children = components_list_query.single_mut();
        let previous_index = event.previous_index();
        let component_index = event.component_index();

        // TODO: move this reordering functionality to list module (send onListReorderRequested event)
        component_list_children.swap(previous_index, component_index);

        for child in component_list_children.iter() {
            let mut component_item = component_item_query.get_mut(*child).unwrap();
            let item_index = component_item.index();

            if item_index == previous_index {
                component_item.set_index(component_index);
            } else if item_index == component_index {
                component_item.set_index(previous_index);
            }
        }
    }
}

fn update_reorder_buttons_on_reorder_event(
    mut on_reordered: EventReader<OnRoadComponentReordered>,
    components_list_query: Query<Entity, With<RoadComponentsList>>,
    mut reorder_button_query: Query<(Entity, &ReorderButton, &mut Visibility)>,
    list_item_query: Query<&ListItem>,
    parent_query: Query<&Parent>,
    active_road: Res<ActiveRoad>,
) {
    for _ in on_reordered.read() {
        let component_list_entity = components_list_query.single();
        let component_count = active_road.component_count();

        update_visibility_of_component_reorder_buttons(
            &mut reorder_button_query,
            &parent_query,
            &list_item_query,
            component_list_entity,
            component_count,
        );
    }
}

fn update_visibility_of_component_reorder_buttons(
    reorder_button_query: &mut Query<(Entity, &ReorderButton, &mut Visibility)>,
    parent_query: &Query<&Parent>,
    list_item_query: &Query<&ListItem>,
    component_list_entity: Entity,
    component_count: usize,
) {
    for (_, reorder_button, mut button_visibility) in
        reorder_button_query
            .iter_mut()
            .filter(|(button_entity, _, _)| {
                entity_is_descendant_of(parent_query, *button_entity, component_list_entity)
            })
    {
        let list_item = list_item_query
            .get(reorder_button.list_item_entity())
            .unwrap();

        update_reorder_button_visibility(
            reorder_button,
            &mut button_visibility,
            component_count,
            list_item.index(),
        );
    }
}

fn update_reorder_button_visibility(
    reorder_button: &ReorderButton,
    button_visibility: &mut Visibility,
    component_count: usize,
    index: usize,
) {
    let target_visibility =
        get_button_target_visibility(reorder_button.direction(), index, component_count);

    if *button_visibility != target_visibility {
        *button_visibility = target_visibility;
    }
}

fn delete_road_component_on_event(
    mut on_deleted: EventReader<OnRoadComponentDeleted>,
    mut commands: Commands,
    mut component_item_query: Query<&mut ListItem, With<RoadComponentItem>>,
) {
    for event in on_deleted.read() {
        let mut component_item_commands = commands.entity(event.component_entity());

        component_item_commands.remove_parent();
        component_item_commands.despawn_recursive();

        for mut item in component_item_query
            .iter_mut()
            .filter(|item| item.index() > event.component_index())
        {
            let new_index = item.index() - 1;
            item.set_index(new_index);
        }
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
        index,
    )));
    let container_entity = container.id();

    container.with_children(|container| {
        container.spawn(build_component_display_node(road_component));
        container.spawn(build_component_text_node(road_component));

        container
            .spawn(build_button_container_node())
            .with_children(|button_container| {
                spawn_reorder_button(
                    button_container,
                    ReorderDirection::Previous,
                    container_entity,
                    26.0,
                    get_button_target_visibility(
                        ReorderDirection::Previous,
                        index,
                        component_count,
                    ),
                );

                spawn_reorder_button(
                    button_container,
                    ReorderDirection::Next,
                    container_entity,
                    26.0,
                    get_button_target_visibility(ReorderDirection::Next, index, component_count),
                );
            });
    });

    container_entity
}

fn get_button_target_visibility(
    reorder_direction: ReorderDirection,
    index: usize,
    component_count: usize,
) -> Visibility {
    let should_be_visible = match reorder_direction {
        ReorderDirection::Next => index < component_count - 1,
        ReorderDirection::Previous => index > 0,
    };

    get_visibility(should_be_visible)
}

fn get_visibility(visible: bool) -> Visibility {
    match visible {
        true => Visibility::default(),
        false => Visibility::Hidden,
    }
}

fn update_component_display(
    style: &mut Style,
    background_color: &mut BackgroundColor,
    road_component: &RoadComponent,
) {
    *style = build_component_display_style(road_component);
    *background_color = road_component.color().into();
}

fn update_component_name(text: &mut Text, road_component: &RoadComponent) {
    text.sections[0].value = road_component.name().to_string();
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
