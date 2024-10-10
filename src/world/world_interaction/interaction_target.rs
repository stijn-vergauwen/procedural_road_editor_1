use bevy::prelude::*;

pub struct InteractionTargetPlugin;

impl Plugin for InteractionTargetPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}

// TODO: add system to check if interaction ray hits target each frame, then set target if something hit

pub struct InteractionTarget {
    pub point: Vec3,
    pub normal: Direction,
}