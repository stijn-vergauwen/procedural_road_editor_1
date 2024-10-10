use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::GameRunningSet;

use super::WorldInteraction;

pub struct WorldInteractionGizmosPlugin;

impl Plugin for WorldInteractionGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            draw_interaction_target_gizmo.in_set(GameRunningSet::UpdateEntities),
        );
    }
}

fn draw_interaction_target_gizmo(world_interaction: Res<WorldInteraction>, mut gizmos: Gizmos) {
    if let Some(target) = &world_interaction.interaction_target {
        gizmos.sphere(
            target.point,
            Transform::default()
                .looking_to(target.normal, Dir3::Y)
                .rotation,
            0.5,
            TEAL_500,
        );
    }
}
