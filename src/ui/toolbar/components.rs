mod reorder;

use bevy::{color::palettes::tailwind::*, prelude::*, text::BreakLineOn};
use reorder::ReorderPlugin;

use crate::{
    road::{OnActiveRoadModified, RoadComponent},
    ui::{
        buttons::{spawn_reorder_button, ReorderDirection},
        ListItem,
    },
    GameRunningSet,
};

use super::RoadComponentsList;

pub struct ToolbarComponentsPlugin;

impl Plugin for ToolbarComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReorderPlugin).add_systems(
            Update,
            generate_road_components.in_set(GameRunningSet::UpdateEntities),
        );
    }
}

pub fn generate_road_components(
    mut on_road_modified: EventReader<OnActiveRoadModified>,
    components_list_query: Query<Entity, With<RoadComponentsList>>,
    mut commands: Commands,
) {
    for event in on_road_modified.read() {
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

fn spawn_road_component(
    components_list: &mut ChildBuilder,
    index: usize,
    components_list_entity: Entity,
    road_component: &RoadComponent,
    component_count: usize,
) {
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
}

fn build_road_components_container_node(list_item: ListItem) -> impl Bundle {
    (
        list_item,
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
    NodeBundle {
        style: Style {
            width: Val::Px(road_component.size().x * 50.0),
            height: Val::Px(road_component.size().y * 50.0),
            ..default()
        },
        background_color: BackgroundColor(GRAY_600.into()),
        ..default()
    }
}

fn build_component_text_node(road_component: &RoadComponent) -> impl Bundle {
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
    }
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
