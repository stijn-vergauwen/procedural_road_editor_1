use bevy::prelude::*;
use bevy_rapier3d::{plugin::RapierContext, prelude::*};

use crate::GameRunningSet;

use super::WorldInteraction;

pub struct InteractionTargetPlugin;

impl Plugin for InteractionTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_interaction_target).after(GameRunningSet::FetchData),
        );
    }
}

// TODO: add system to check if interaction ray hits target each frame, then set target if something hit

#[derive(Debug)]
pub struct InteractionTarget {
    pub point: Vec3,
    pub normal: Dir3,
}

impl InteractionTarget {
    fn from_raycast(intersection: RayIntersection) -> Self {
        Self {
            point: intersection.point,
            normal: Dir3::new(intersection.normal).unwrap_or(Dir3::Y),
        }
    }
}

fn update_interaction_target(
    mut world_interaction: ResMut<WorldInteraction>,
    rapier_context: Res<RapierContext>,
) {
    world_interaction.interaction_target =
        calculate_interaction_target(&rapier_context, &world_interaction);
}

// Utility

fn calculate_interaction_target(
    rapier_context: &RapierContext,
    world_interaction: &WorldInteraction,
) -> Option<InteractionTarget> {
    let intersection = raycast_from_ray(
        rapier_context,
        world_interaction.interaction_ray?,
        100.0, // TODO: replace with config
    )?;

    Some(InteractionTarget::from_raycast(intersection))
}

fn raycast_from_ray(
    rapier_context: &RapierContext,
    ray: Ray3d,
    max_distance: f32,
) -> Option<RayIntersection> {
    let intersection = rapier_context.cast_ray_and_get_normal(
        ray.origin,
        ray.direction.as_vec3(),
        max_distance,
        false,
        QueryFilter::new(),
    );

    intersection.map(|(_, intersection)| intersection)
}
