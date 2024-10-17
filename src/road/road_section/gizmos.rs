use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{game_modes::GameMode, road::road_node::RoadNode, GameRunningSet};

use super::RoadSection;

const ROAD_SECTION_GIZMO_COLOR: Srgba = ORANGE_500;
const ROAD_SECTION_GIZMO_WIDTH: f32 = 8.0;
const ROAD_SECTION_GIZMO_HEIGHT: f32 = 1.0;

pub struct RoadSectionGizmosPlugin;

impl Plugin for RoadSectionGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            draw_road_section_gizmos
                .in_set(GameRunningSet::DrawGizmos)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn draw_road_section_gizmos(
    mut gizmos: Gizmos,
    road_section_query: Query<&RoadSection>,
    road_node_query: Query<&Transform, With<RoadNode>>,
) {
    for section in road_section_query.iter() {
        let Ok(start_node_transform) = road_node_query.get(section.start_node) else {
            continue;
        };

        let Ok(end_node_transform) = road_node_query.get(section.end_node) else {
            continue;
        };

        let section_transform = calculate_road_section_gizmo_transform(
            start_node_transform.translation,
            end_node_transform.translation,
        );

        gizmos.cuboid(section_transform, ROAD_SECTION_GIZMO_COLOR);
    }
}

pub fn calculate_road_section_gizmo_transform(
    start_node_position: Vec3,
    end_node_position: Vec3,
) -> Transform {
    let delta_position = end_node_position - start_node_position;
    let section_position = start_node_position + delta_position / 2.0;
    let section_rotation = Transform::from_translation(start_node_position)
        .looking_at(end_node_position, Dir3::Y)
        .rotation;

    let height_adjusted_position = section_position + Vec3::Y * (ROAD_SECTION_GIZMO_HEIGHT / 2.0);

    Transform {
        translation: height_adjusted_position,
        rotation: section_rotation,
        // TODO: this scale should be just splat(1.0) once road section meshes are the actual road designs. Currently we're saying each section is a cube which is deformed by this scale vector
        scale: Vec3::new(
            ROAD_SECTION_GIZMO_WIDTH,
            ROAD_SECTION_GIZMO_HEIGHT,
            start_node_position.distance(end_node_position),
        ),
    }
}
