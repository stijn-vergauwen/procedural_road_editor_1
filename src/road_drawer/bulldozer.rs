use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    road::road_section::RoadSection,
    world::world_interaction::{
        mouse_interaction_events::{InteractionPhase, OnMouseInteraction},
        WorldInteraction,
    },
    GameRunningSet,
};

use super::RoadDrawerTool;

pub struct BulldozerPlugin;

impl Plugin for BulldozerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                switch_to_bulldozer_on_b_key.in_set(GameRunningSet::GetUserInput),
                delete_road_sections_that_are_clicked_on
                    .in_set(GameRunningSet::UpdateEntities)
                    .run_if(in_state(RoadDrawerTool::Bulldozer)),
            )
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn switch_to_bulldozer_on_b_key(
    current_tool: Res<State<RoadDrawerTool>>,
    mut next_tool: ResMut<NextState<RoadDrawerTool>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        let new_value = match current_tool.get() {
            RoadDrawerTool::Drawer => RoadDrawerTool::Bulldozer,
            RoadDrawerTool::Bulldozer => RoadDrawerTool::Drawer,
        };

        println!("Switch to tool: {:?}", new_value);

        next_tool.set(new_value);
    }
}

fn delete_road_sections_that_are_clicked_on(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut commands: Commands,
    world_interaction: Res<WorldInteraction>,
    road_section_query: Query<&RoadSection>,
) {
    for _ in on_interaction.read().filter(|event| {
        event.button == MouseButton::Left
            && event.phase == InteractionPhase::Started
            && !event.is_on_ui
    }) {
        let Some(interaction_target) = world_interaction.interaction_target() else {
            continue;
        };

        delete_road_section(
            interaction_target.entity,
            &mut commands,
            &road_section_query,
        );
    }
}

fn delete_road_section(
    road_section_to_delete: Entity,
    commands: &mut Commands,
    road_section_query: &Query<&RoadSection>,
) {
    let Ok(road_section) = road_section_query.get(road_section_to_delete).cloned() else {
        return;
    };

    commands.entity(road_section_to_delete).despawn();

    if count_sections_connected_to_road_node(road_section.start_node, road_section_query) == 1 {
        commands.entity(road_section.start_node).despawn();
    }

    if count_sections_connected_to_road_node(road_section.end_node, road_section_query) == 1 {
        commands.entity(road_section.end_node).despawn();
    }
}

fn count_sections_connected_to_road_node(
    road_node: Entity,
    road_section_query: &Query<&RoadSection>,
) -> u8 {
    let mut result = 0;

    for road_section in road_section_query.iter() {
        if road_section.start_node == road_node {
            result += 1;
        }

        if road_section.end_node == road_node {
            result += 1;
        }
    }

    result
}
