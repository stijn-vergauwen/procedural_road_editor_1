mod reorder;

use bevy::{color::palettes::tailwind::*, prelude::*, text::BreakLineOn};
use reorder::ReorderPlugin;

use crate::{
    road::OnActiveRoadModified,
    ui::{
        buttons::{build_reorder_button, ReorderDirection},
        ListItem,
    },
    GameRunningSet,
};

use super::ToolBar;

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
    toolbar_query: Query<Entity, With<ToolBar>>,
    mut commands: Commands,
) {
    for event in on_road_modified.read() {
        let toolbar_entity = toolbar_query.single();

        commands
            .entity(toolbar_entity)
            .despawn_descendants()
            .with_children(|toolbar| {
                let road_components = event.road().components();
                for (index, road_component) in road_components.iter().enumerate() {
                    let container_node = (
                        ListItem::new(toolbar_entity, index as u8),
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
                    );

                    let display_node = NodeBundle {
                        style: Style {
                            width: Val::Px(road_component.size().x * 50.0),
                            height: Val::Px(road_component.size().y * 50.0),
                            ..default()
                        },
                        background_color: BackgroundColor(GRAY_600.into()),
                        ..default()
                    };

                    let text_node = TextBundle {
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
                    };

                    let button_container_node = NodeBundle {
                        style: Style {
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    };

                    let mut container = toolbar.spawn(container_node);
                    let container_entity = container.id();

                    container.with_children(|container| {
                        container.spawn(display_node);
                        container.spawn(text_node);

                        container
                            .spawn(button_container_node)
                            .with_children(|button_container| {
                                if index > 0 {
                                    build_reorder_button(
                                        button_container,
                                        ReorderDirection::Previous,
                                        container_entity,
                                        26.0,
                                    );
                                }

                                if index < road_components.len() - 1 {
                                    build_reorder_button(
                                        button_container,
                                        ReorderDirection::Next,
                                        container_entity,
                                        26.0,
                                    );
                                }
                            });
                    });
                }
            });
    }
}
