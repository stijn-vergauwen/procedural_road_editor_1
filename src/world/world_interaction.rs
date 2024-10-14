mod gizmos;
pub mod interaction_target;
pub mod mouse_interaction_events;

use bevy::{prelude::*, window::PrimaryWindow};
use gizmos::WorldInteractionGizmosPlugin;
use interaction_target::{InteractionTarget, InteractionTargetPlugin};
use mouse_interaction_events::MouseInteractionEventsPlugin;

use crate::{utility::distance::Distance, GameRunningSet};

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InteractionTargetPlugin,
            WorldInteractionGizmosPlugin,
            MouseInteractionEventsPlugin,
        ))
        .add_event::<OnWorldInteractionRayUpdated>()
        .insert_resource(WorldInteraction::default())
        .add_systems(
            Update,
            update_interaction_ray.in_set(GameRunningSet::FetchData),
        );
    }
}

#[derive(Resource, Debug)]
pub struct WorldInteraction {
    interaction_ray: Option<Ray3d>,
    interaction_target: Option<InteractionTarget>,
    config: WorldInteractionConfig,
}

impl WorldInteraction {
    pub fn interaction_target(&self) -> Option<InteractionTarget> {
        self.interaction_target
    }
}

impl Default for WorldInteraction {
    fn default() -> Self {
        Self {
            interaction_ray: None,
            interaction_target: None,
            config: WorldInteractionConfig::new(Distance(150.0)),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WorldInteractionConfig {
    pub max_interaction_distance: Distance,
}

impl WorldInteractionConfig {
    pub fn new(max_interaction_distance: Distance) -> Self {
        Self {
            max_interaction_distance,
        }
    }
}

#[derive(Event, Clone, Copy)]
pub struct OnWorldInteractionRayUpdated {
    pub interaction_ray: Option<Ray3d>,
}

impl OnWorldInteractionRayUpdated {
    pub fn new(interaction_ray: Option<Ray3d>) -> Self {
        Self { interaction_ray }
    }
}

fn update_interaction_ray(
    mut on_updated: EventWriter<OnWorldInteractionRayUpdated>,
    mut world_interaction: ResMut<WorldInteraction>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let Ok((camera, camera_global_transform)) = camera_query.get_single() else {
        return;
    };

    let new_ray = get_ray_through_cursor(camera, camera_global_transform, window);

    if world_interaction.interaction_ray != new_ray {
        world_interaction.interaction_ray = new_ray;
        on_updated.send(OnWorldInteractionRayUpdated::new(new_ray));
    }
}

// Utility

fn get_ray_through_cursor(
    camera: &Camera,
    camera_global_transform: &GlobalTransform,
    window: &Window,
) -> Option<Ray3d> {
    camera.viewport_to_world(camera_global_transform, window.cursor_position()?)
}
