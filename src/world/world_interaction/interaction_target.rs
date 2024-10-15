use bevy::prelude::*;
use bevy_rapier3d::{plugin::RapierContext, prelude::*};

use crate::GameRunningSet;

use super::{OnWorldInteractionRayUpdated, WorldInteraction};

pub struct InteractionTargetPlugin;

impl Plugin for InteractionTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnWorldInteractionTargetUpdated>()
            .add_systems(
                Update,
                (update_interaction_target).after(GameRunningSet::FetchData),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InteractionTarget {
    pub point: Vec3,
    pub normal: Dir3,
    pub entity: Entity,
}

impl InteractionTarget {
    fn from_raycast(intersection: RayIntersection, entity: Entity) -> Self {
        Self {
            point: intersection.point,
            normal: Dir3::new(intersection.normal).unwrap_or(Dir3::Y),
            entity,
        }
    }
}

#[derive(Event, Clone, Copy)]
pub struct OnWorldInteractionTargetUpdated {
    pub interaction_target: Option<InteractionTarget>,
}

impl OnWorldInteractionTargetUpdated {
    pub fn new(interaction_target: Option<InteractionTarget>) -> Self {
        Self { interaction_target }
    }
}

fn update_interaction_target(
    mut on_ray_updated: EventReader<OnWorldInteractionRayUpdated>,
    mut on_target_updated: EventWriter<OnWorldInteractionTargetUpdated>,
    mut world_interaction: ResMut<WorldInteraction>,
    rapier_context: Res<RapierContext>,
) {
    for event in on_ray_updated.read() {
        let new_target = calculate_interaction_target(
            &rapier_context,
            &world_interaction,
            event.interaction_ray,
        );

        if world_interaction.interaction_target != new_target {
            world_interaction.interaction_target = new_target;
            on_target_updated.send(OnWorldInteractionTargetUpdated::new(new_target));
        }
    }
}

// Utility

fn calculate_interaction_target(
    rapier_context: &RapierContext,
    world_interaction: &WorldInteraction,
    interaction_ray: Option<Ray3d>,
) -> Option<InteractionTarget> {
    let max_distance = world_interaction
        .config
        .max_interaction_distance
        .as_meters();

    raycast_from_ray(rapier_context, interaction_ray?, max_distance)
        .map(|(entity, intersection)| InteractionTarget::from_raycast(intersection, entity))
}

fn raycast_from_ray(
    rapier_context: &RapierContext,
    ray: Ray3d,
    max_distance: f32,
) -> Option<(Entity, RayIntersection)> {
    rapier_context.cast_ray_and_get_normal(
        ray.origin,
        ray.direction.as_vec3(),
        max_distance,
        false,
        QueryFilter::new(),
    )
}
