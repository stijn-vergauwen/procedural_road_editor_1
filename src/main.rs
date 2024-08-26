mod road;
mod schedule;
mod ui;
mod utility;
mod world;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use road::RoadPlugin;
pub use schedule::GameRunningSet;
use schedule::SchedulePlugin;
use ui::UiPlugin;
use utility::UtilityPlugin;
use world::WorldPlugin;

/*
    Project goals:
        A 3d road editor where you can create custom road designs from basic building blocks like:
        - Lanes, medians, sidewalks
        - Lines & other markings
        Widths, heights, colors are configurable. With support for spaced elements like drains or trees.
        These designs can then be saved as files, and loaded back in.

        The app has 2 sides:
        - Editor where road designs can be made or altered
        - "In-game" where you can select your road designs and draw roads with them


    Things to practice:
        - UI, especially displaying lists, using icons, collapsable menu's, inputs
        - Building an 'editor' like environment / tool
        - Procedural geometry and textures
        - Serializing custom data structures


    Future idea's:
        - Thumbnail images of roads
        - Transitions between roads
        - Intersections
            - connecting lane lines
            - editing traffic directions / flow
            - toggles for road signs or traffic lights
        - Connecting a traffic simulation
        - Expanding the road building tools, roundabouts, consistent angles and corners, etc etc


    Code standards:
        - Events EVERYWHERE, everything that doesn't happen every frame or across frames


    Backlog:
        - Lane lines support <- doing
        - Goal: Have a multilane highway road design with grass median
        - General road markings support
        - Spaced elements support
        - Support for assets in spaced elements
        - Goal: Have a road design that includes trees

*/

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            UtilityPlugin,
            SchedulePlugin,
            WorldPlugin,
            RoadPlugin,
            UiPlugin,
        ))
        .run();
}
