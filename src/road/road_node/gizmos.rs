use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{game_modes::GameMode, road::road_node::RoadNode, GameRunningSet};

const ROAD_NODE_GIZMO_COLOR: Srgba = EMERALD_600;
const ROAD_NODE_GIZMO_RADIUS: f32 = 2.0;
const ROAD_NODE_GIZMO_HEIGHT: f32 = 1.0;

pub struct RoadNodeGizmosPlugin;

impl Plugin for RoadNodeGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            draw_road_node_gizmos
                .in_set(GameRunningSet::DrawGizmos)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn draw_road_node_gizmos(mut gizmos: Gizmos, road_node_query: Query<&Transform, With<RoadNode>>) {
    for node_transform in road_node_query.iter() {
        let position = node_transform.translation;

        draw_road_node_gizmo(&mut gizmos, position, ROAD_NODE_GIZMO_COLOR);
    }
}

pub fn draw_road_node_gizmo(gizmos: &mut Gizmos, position: Vec3, color: impl Into<Color> + Clone) {
    draw_circle_gizmo(gizmos, position, color.clone());
    draw_circle_gizmo(gizmos, position.with_y(ROAD_NODE_GIZMO_HEIGHT), color);
}

fn draw_circle_gizmo(gizmos: &mut Gizmos, position: Vec3, color: impl Into<Color>) {
    gizmos.circle(position, Dir3::Y, ROAD_NODE_GIZMO_RADIUS, color);
}
