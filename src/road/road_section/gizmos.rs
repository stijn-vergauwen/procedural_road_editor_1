use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    game_modes::GameMode,
    road::{road_data::RoadData, road_node::RoadNode},
    utility::circular_arc::CircularArc,
    GameRunningSet,
};

use super::{
    calculate_road_section_size, calculate_road_section_transform, RoadSection, RoadSectionVariant,
};

const ROAD_SECTION_GIZMO_COLOR: Srgba = SKY_500;
const CURVED_SECTION_TRANSFORM_COUNT_MULTIPLIER: f32 = 0.5;

pub struct RoadSectionGizmosPlugin;

impl Plugin for RoadSectionGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            draw_gizmos
                .in_set(GameRunningSet::DrawGizmos)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    road_section_query: Query<&RoadSection>,
    road_node_query: Query<&Transform, With<RoadNode>>,
) {
    for section in road_section_query.iter() {
        draw_road_section_gizmo(
            &mut gizmos,
            section,
            ROAD_SECTION_GIZMO_COLOR,
            &road_node_query,
        );
    }
}

fn draw_road_section_gizmo(
    gizmos: &mut Gizmos,
    road_section: &RoadSection,
    color: impl Into<Color> + Clone,
    road_node_query: &Query<&Transform, With<RoadNode>>,
) {
    match road_section.variant {
        RoadSectionVariant::Straight => {
            let Ok(start_node_transform) =
                road_node_query.get(road_section.start().road_node_entity)
            else {
                return;
            };

            let Ok(end_node_transform) = road_node_query.get(road_section.end().road_node_entity)
            else {
                return;
            };

            draw_straight_road_section_gizmo(
                gizmos,
                &road_section.road_design,
                start_node_transform.translation,
                end_node_transform.translation,
                color,
            )
        }
        RoadSectionVariant::Curved(circular_arc) => {
            draw_curved_road_section_gizmo(gizmos, &road_section.road_design, &circular_arc, color)
        }
    }
}

pub fn draw_straight_road_section_gizmo(
    gizmos: &mut Gizmos,
    road_design: &RoadData,
    start_node_position: Vec3,
    end_node_position: Vec3,
    color: impl Into<Color> + Clone,
) {
    let section_transform = calculate_road_section_gizmo_transform(
        &road_design,
        start_node_position,
        end_node_position,
    );

    gizmos.cuboid(section_transform, color);
}

pub fn draw_curved_road_section_gizmo(
    gizmos: &mut Gizmos,
    road_design: &RoadData,
    circular_arc: &CircularArc,
    color: impl Into<Color> + Clone,
) {
    let half_road_width = road_design.half_width();
    let transform_count =
        ((circular_arc.length().abs() * CURVED_SECTION_TRANSFORM_COUNT_MULTIPLIER).round() as u32)
            .max(5);
    let transform_direction = circular_arc.forwards_direction();

    let mut previous_transform: Option<Transform> = None;

    for transform_along_arc in
        circular_arc.calculate_transforms_along_arc(transform_count, transform_direction)
    {
        let offset_points = get_transform_offset_points(transform_along_arc, half_road_width);

        gizmos.line(offset_points.0, offset_points.1, color.clone());

        if let Some(transform) = previous_transform {
            let previous_offset_points = get_transform_offset_points(transform, half_road_width);

            gizmos.line(previous_offset_points.0, offset_points.0, color.clone());
            gizmos.line(previous_offset_points.1, offset_points.1, color.clone());
        }

        previous_transform = Some(transform_along_arc);
    }

    fn get_transform_offset_points(transform: Transform, half_road_width: f32) -> (Vec3, Vec3) {
        (
            transform.translation + transform.left() * half_road_width,
            transform.translation + transform.right() * half_road_width,
        )
    }
}

fn calculate_road_section_gizmo_transform(
    road_design: &RoadData,
    start_node_position: Vec3,
    end_node_position: Vec3,
) -> Transform {
    let mut section_transform =
        calculate_road_section_transform(start_node_position, end_node_position);

    section_transform.translation.y += road_design.total_height() / 2.0;
    section_transform.scale =
        calculate_road_section_size(road_design, start_node_position, end_node_position);

    section_transform
}
