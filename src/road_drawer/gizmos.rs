use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{game_modes::GameMode, GameRunningSet};

use super::RoadDrawer;

const ROAD_NODE_GIZMO_COLOR: Srgba = CYAN_400;
const ROAD_NODE_GIZMO_SCALE: f32 = 0.5;

pub struct RoadDrawerGizmosPlugin;

impl Plugin for RoadDrawerGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (draw_gizmos_for_road_being_drawn.in_set(GameRunningSet::DrawGizmos),)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn draw_gizmos_for_road_being_drawn(mut gizmos: Gizmos, road_drawer: Res<RoadDrawer>) {
    if let Some(road_being_drawn) = road_drawer.section_being_drawn {
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
