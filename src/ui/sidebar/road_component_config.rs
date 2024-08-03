use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::ui::{build_text_node, inputs::number_input::spawn_number_input_node};

use super::Sidebar;

pub struct RoadComponentConfigPlugin;

impl Plugin for RoadComponentConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, generate_config_section);
    }
}

#[derive(Component)]
pub struct RoadComponentConfig;

// TODO: change this to react to events instead of running on PostStartup
// TODO: remove previous children of sidebar before spawning
fn generate_config_section(mut commands: Commands, sidebar_query: Query<Entity, With<Sidebar>>) {
    let sidebar = sidebar_query.single();

    commands.entity(sidebar).with_children(|sidebar| {
        sidebar
            .spawn(build_config_container_node())
            .with_children(|container| {
                container.spawn(build_text_node(
                    "Title",
                    24.0,
                    GRAY_900,
                    JustifyText::Center,
                    (),
                ));
                spawn_number_input_node(container, "Width", 0.0, 0.0..10.0);
                spawn_number_input_node(container, "Height", 0.0, 0.0..10.0);
            });
    });
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
