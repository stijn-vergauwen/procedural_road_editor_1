pub mod selected_road;

use bevy::{color::palettes::tailwind::*, prelude::*};
use selected_road::SelectedRoadPlugin;

use crate::{
    game_modes::GameMode,
    world::world_interaction::{
        mouse_interaction_events::{InteractionPhase, OnMouseInteraction},
        WorldInteraction,
    },
    GameRunningSet,
};

const MOUSE_BUTTON_TO_DRAW: MouseButton = MouseButton::Left;

const ROAD_NODE_GIZMO_COLOR: Srgba = CYAN_400;
const ROAD_NODE_GIZMO_SCALE: f32 = 0.5;

pub struct RoadDrawerPlugin;

impl Plugin for RoadDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectedRoadPlugin)
            .init_resource::<RoadDrawer>()
            .add_systems(
                Update,
                (
                    start_drawing_road_on_mouse_interaction.in_set(GameRunningSet::UpdateEntities),
                    draw_gizmos_for_road_being_drawn.in_set(GameRunningSet::DrawGizmos),
                )
                    .run_if(in_state(GameMode::RoadDrawer)),
            );
    }
}

// TODO: only have resource exist in RoadDrawer mode
#[derive(Resource, Default, Debug)]
pub struct RoadDrawer {
    road_being_drawn: Option<RoadBeingDrawn>,
}

#[derive(Clone, Copy, Debug)]
struct RoadBeingDrawn {
    start: RoadNodeBeingDrawn,
    end: RoadNodeBeingDrawn,
}

#[derive(Clone, Copy, Debug)]
struct RoadNodeBeingDrawn {
    position: Vec3,
    /// Entity of an existing RoadNode if the road starts / ends on an existing node, otherwise None.
    existing_node_entity: Option<Entity>,
}

impl RoadNodeBeingDrawn {
    fn new(position: Vec3, existing_node_entity: Option<Entity>) -> Self {
        Self {
            position,
            existing_node_entity,
        }
    }
}

// TODO: split to module
fn start_drawing_road_on_mouse_interaction(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
) {
    // TODO: early return when mouse in on UI

    let Some(interaction_target) = world_interaction.interaction_target() else {
        return;
    };

    for _ in on_interaction.read().filter(|event| {
        event.button == MOUSE_BUTTON_TO_DRAW && event.phase == InteractionPhase::Started
    }) {
        let interaction_position = interaction_target.point;

        road_drawer.road_being_drawn = Some(RoadBeingDrawn {
            start: RoadNodeBeingDrawn::new(interaction_position, None),
            end: RoadNodeBeingDrawn::new(interaction_position, None),
        });
    }
}

// TODO: split to module
fn draw_gizmos_for_road_being_drawn(mut gizmos: Gizmos, road_drawer: Res<RoadDrawer>) {
    if let Some(road_being_drawn) = road_drawer.road_being_drawn {
        if road_being_drawn.start.existing_node_entity.is_none() {
            draw_road_node_gizmo(&mut gizmos, road_being_drawn.start.position);
        }
        draw_road_node_gizmo(&mut gizmos, road_being_drawn.end.position);
    }
}

fn draw_road_node_gizmo(gizmos: &mut Gizmos, position: Vec3) {
    gizmos.cuboid(
        Transform::from_translation(position).with_scale(Vec3::splat(ROAD_NODE_GIZMO_SCALE)),
        ROAD_NODE_GIZMO_COLOR,
    );
}
