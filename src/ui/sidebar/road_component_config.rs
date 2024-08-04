use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    ui::{
        build_text_node,
        inputs::number_input::spawn_number_input_node,
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
                generate_config_section_for_selected_component,
                despawn_config_section_on_component_deselected,
            )
                .in_set(GameRunningSet::DespawnEntities),
        );
    }
}

#[derive(Component)]
pub struct RoadComponentConfig;

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
                sidebar
                    .spawn(build_config_container_node())
                    .with_children(|container| {
                        container.spawn(build_text_node(
                            component_data.name(),
                            30.0,
                            GRAY_900,
                            JustifyText::Center,
                            (),
                        ));
                        spawn_number_input_node(
                            container,
                            "Width",
                            component_data.size().x,
                            0.0..10.0,
                        );
                        spawn_number_input_node(
                            container,
                            "Height",
                            component_data.size().y,
                            0.0..10.0,
                        );
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

fn build_config_container_node() -> impl Bundle {
    (
        RoadComponentConfig,
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
