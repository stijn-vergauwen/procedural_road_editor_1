mod schedule;
mod utility;
mod world;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub use schedule::GameRunningSet;
use schedule::SchedulePlugin;
use utility::UtilityPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            UtilityPlugin,
            SchedulePlugin,
            WorldPlugin,
        ))
        .run();
}
