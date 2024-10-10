mod interaction_target;

use bevy::prelude::*;
use interaction_target::InteractionTarget;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInteractionPlugin);
    }
}

// TODO: add worldInteractionConfig for max distance
// TODO: add system to calculate the ray from camera each frame

#[derive(Resource)]
pub struct WorldInteraction {
    interaction_ray: Option<Ray3d>,
    interaction_target: Option<InteractionTarget>,
}

fn update_interaction_ray(mut world_interaction: ResMut<WorldInteraction>) {
    // TODO: implement
}
