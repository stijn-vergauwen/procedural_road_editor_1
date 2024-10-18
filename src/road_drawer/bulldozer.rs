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

        // TODO: also delete road nodes that aren't connected anymore

        if road_section_query.contains(interaction_target.entity) {
            println!("Delete road section");

            commands
                .entity(interaction_target.entity)
                .despawn_recursive();
        }
    }
}
