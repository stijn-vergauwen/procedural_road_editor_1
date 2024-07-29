use bevy::{color::palettes::tailwind::*, prelude::*, text::BreakLineOn};

use crate::{road::OnActiveRoadModified, GameRunningSet};

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_toolbar).add_systems(
            Update,
            generate_road_components.in_set(GameRunningSet::UpdateEntities),
        );
    }
}

#[derive(Component)]
struct ToolBar;

fn spawn_toolbar(mut commands: Commands) {
    let container_node = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Stretch,
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    };

    let toolbar_node = NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            column_gap: Val::Px(16.0),
            ..default()
        },
        background_color: BackgroundColor(CYAN_800.with_alpha(0.4).into()),
        ..default()
    };

    commands.spawn(container_node).with_children(|container| {
        container.spawn((toolbar_node, ToolBar));
    });
}

fn generate_road_components(
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
                for road_component in event.road().components() {
                    let road_component_container_node = NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::End,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    };

                    let road_component_display_node = NodeBundle {
                        style: Style {
                            width: Val::Px(road_component.size().x * 50.0),
                            height: Val::Px(road_component.size().y * 50.0),
                            ..default()
                        },
                        background_color: BackgroundColor(GRAY_600.into()),
                        ..default()
                    };

                    let road_component_text_node = TextBundle {
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

                    toolbar
                        .spawn(road_component_container_node)
                        .with_children(|container| {
                            container.spawn(road_component_display_node);
                            container.spawn(road_component_text_node);
                        });
                }
            });
    }
}
