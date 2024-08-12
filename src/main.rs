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
        Widths, heights, colors are configurable. With support for spaced elements like gutters or trees.
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
        - Large refactor <- doing
            - Something is going wrong when deleting components, the index seems to be wrong (deletes last element?)
            - Recalculate reorder button visibility after component is added
            - Move modules in "buttons" module to the code that actually uses them
            - Give reorder button module it's own plugin
                - Move button visibility logic from components module to the reorder button module
            - Decide structure for what components should have entity references and what shouldn't
                - Make a "Partial" component that holds an entity ref (? so refs for connecting groups of entities, but no refs for checking heirarchy?)
                - Remove all unwanted entity refs
            - Make UI utility modules
                - Node style templates (consts with presets for Style values, pretty sure this way you can overwrite what you want and fill the other values in the same way as "Default")
            - Rework layout & colors etc, more web-devvy

        - Lane lines support
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
