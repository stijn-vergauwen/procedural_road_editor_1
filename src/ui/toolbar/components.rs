mod reorder;
pub mod selected_road_component;

use bevy::{color::palettes::tailwind::*, prelude::*};
use reorder::ReorderPlugin;
use selected_road_component::{
    OnRoadComponentDeselected, OnRoadComponentSelected, SelectedRoadComponentPlugin,
};

use crate::{
    road::{
        active_road::{
            new_road_component::OnRoadComponentAdded,
            road_component_change::OnRoadComponentChanged, OnActiveRoadSet,
        },
        RoadComponent,
    },
    ui::{
        components::{
            flexbox::{FlexboxBuilder, FlexboxConfig},
            text::{TextBuilder, TextConfig},
            UiComponentBuilder, UiComponentWithChildrenBuilder,
        },
        list::{
            reorder_button::{spawn_reorder_button, ReorderDirection},
            ListItem,
        },
    },
    utility::find_descendant_of_entity_mut,
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
                    (add_road_component_on_event, update_road_component_on_change)
                        .in_set(GameRunningSet::UpdateEntities),
                    (rebuild_road_components_on_active_road_set,)
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

        commands
            .entity(components_list_entity)
            .despawn_descendants()
            .with_children(|components_list| {
                for (index, road_component) in road_components.iter().enumerate() {
                    spawn_road_component(components_list, index, road_component);
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

        commands
            .entity(components_list_entity)
            .with_children(|components_list| {
                let component_item_entity = spawn_road_component(
                    components_list,
                    event.component_index(),
                    event.component_data(),
                );

                on_component_selected.send(OnRoadComponentSelected::new(
                    event.component_index(),
                    component_item_entity,
                ));
            });
    }
}

fn update_road_component_on_change(
    mut on_changed: EventReader<OnRoadComponentChanged>,
    mut component_display_query: Query<
        (Entity, &mut Style, &mut BackgroundColor),
        With<RoadComponentDisplay>,
    >,
    mut component_name_query: Query<(Entity, &mut Text), With<RoadComponentName>>,
    children_query: Query<&Children>,
) {
    for event in on_changed.read() {
        let component_entity = event.component_entity();
        let road_component = event.component_data();

        if let Some((_, mut style, mut background_color)) = find_descendant_of_entity_mut(
            component_entity,
            &mut component_display_query,
            |item| item.0,
            &children_query,
        ) {
            update_component_display(&mut style, &mut background_color, road_component);
        }

        if let Some((_, mut text)) = find_descendant_of_entity_mut(
            component_entity,
            &mut component_name_query,
            |item| item.0,
            &children_query,
        ) {
            update_component_name(&mut text, road_component);
        }
    }
}
// Utility

fn spawn_road_component(
    components_list: &mut ChildBuilder,
    index: usize,
    road_component: &RoadComponent,
) -> Entity {
    let road_component_components = (
        ListItem::new(index),
        RoadComponentItem::default(),
        Interaction::default(),
    );

    FlexboxBuilder::new(
        FlexboxConfig::horizontally_centered_column()
            .with_justify(JustifyContent::End)
            .with_px_gap(4.0),
    )
    .spawn(
        components_list,
        road_component_components,
        |road_component_container| {
            road_component_container.spawn(build_component_display_node(road_component));

            spawn_road_component_name(road_component_container, road_component);

            FlexboxBuilder::new(FlexboxConfig::row().with_px_gap(8.0)).spawn(
                road_component_container,
                (),
                |button_container| {
                    spawn_reorder_button(button_container, ReorderDirection::Previous, 26.0);

                    spawn_reorder_button(button_container, ReorderDirection::Next, 26.0);
                },
            );
        },
    )
}

fn spawn_road_component_name(builder: &mut ChildBuilder, road_component: &RoadComponent) -> Entity {
    TextBuilder::new(
        TextConfig::default()
            .with_text(road_component.name())
            .with_justify(JustifyText::Center)
            .clone(),
    )
    .spawn(builder, RoadComponentName)
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

fn build_component_display_node(road_component: &RoadComponent) -> impl Bundle {
    (
        RoadComponentDisplay,
        NodeBundle {
            style: build_component_display_style(road_component),
            background_color: road_component.color().into(),
            border_color: BorderColor::from(NEUTRAL_400),
            ..default()
        },
    )
}

fn build_component_display_style(road_component: &RoadComponent) -> Style {
    Style {
        width: Val::Px(road_component.size().x * COMPONENT_DISPLAY_SCALE),
        height: Val::Px(road_component.size().y * COMPONENT_DISPLAY_SCALE + 1.0),
        border: UiRect::all(Val::Px(1.6)),
        ..default()
    }
}
