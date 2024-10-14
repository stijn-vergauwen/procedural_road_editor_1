mod game_modes;
mod road;
mod road_drawer;
mod road_editor;
mod system_sets;
mod ui;
mod utility;
mod world;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use game_modes::GameModesPlugin;
use road::RoadPlugin;
use road_drawer::RoadDrawerPlugin;
use road_editor::RoadEditorPlugin;
pub use system_sets::GameRunningSet;
use system_sets::SystemSetPlugin;
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
        - Undo functionality
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
        - add 'bulldozer' tool
            - add 'drawer tool' enum
            - button to switch bulldozer tool
            - delete road sections when clicking on them
        - snap to road section ends when drawing
        - draw mesh of selected road for sections
        - curved roads
        - debug gizmos for curved roads
        - align curved sections with connected sections
    
        - Lane marking support
            - config ui (new sidebar screen for markings)
            - lane line events & handlers
            - spaced markings
        - Goal: Have a multilane highway road design with grass median
        - General road markings support
        - Road component presets to choose from when adding road component
            - Open list on "add" button clicked
            - Add selected preset
        - Spaced elements support
        - Support for assets in spaced elements
        - Goal: Have a road design that includes trees



*/

/*
    Ideal project file structure:

    - main
        - road
            - road design -> renamed from road data, since the data is specifically the design
                - road design component
                - road marking
            - road component position
            - road builder
            - road section -> describes a road piece in 3D and what nodes it connects
            - road node -> points in 3D that are connected by sections, would also be used in road transitions and intersections

        - road editor
            - active road
                - road marking
                    - update road markings
                    - events
                        - show road marking config
                        - hide road marking config
                - events
                    - load active road
                    - save active road
                    - set active road
                    - add road component
                    - change road component
                    - delete road component
                    - reorder road component
                    - utility -> for all shared logic (can move some stuff out of ActiveRoad or RoadData)
                        - changed component indices
                - bottom ui
                    - road components list
                    - actions
                - selected component ui
                    - events
                        - select road component
            - road preview
            - sidebar

        - road drawer

        - world
            - camera
            - world interaction

        - main menu

        - utility
            - ui components
                - content wrap, content size, flexbox, section etc
                - inputs
                    - text, number, slider, color
                - buttons
                    - button
                    - text button
                - text
                - modal
                - list
                    - events
                    - reorder button
            - system sets
            - states
            - mesh builder
            - texture builder
            - changed value
            - entity hierarchy (find ascendant descendant etc)

    This would group related code together much better than how it is currently,
    and make it easier to refactor and have each module only responsible for 1 thing.

    I didn't realise how much logic will only be used in either the editor or drawer parts of this project.
    I also didn't realise how much logic will only be relevant to CRUD operations,
    I tend to put CRUD code together with it's data, but currently I think it's better to put it together with code that activates this logic (so in the road editor events)
*/

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            UtilityPlugin,
            SystemSetPlugin,
            WorldPlugin,
            RoadPlugin,
            UiPlugin,
            GameModesPlugin,
            RoadEditorPlugin,
            RoadDrawerPlugin,
        ))
        .run();
}
