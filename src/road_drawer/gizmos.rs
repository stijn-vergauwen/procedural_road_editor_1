use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    game_modes::GameMode,
    road::{
        road_node::gizmos::draw_road_node_gizmo,
        road_section::gizmos::calculate_road_section_gizmo_transform,
    },
    utility::line_intersection::calculate_line_line_intersection_3d,
    GameRunningSet,
};

use super::{
    section_being_drawn::SectionBeingDrawnVariant, selected_road::SelectedRoad, RoadDrawer,
};

const ROAD_NODE_GIZMO_COLOR: Srgba = EMERALD_300;
const ROAD_SECTION_GIZMO_COLOR: Srgba = SKY_300;
const ROAD_SECTION_DIRECTION_GIZMO_COLOR: Srgba = PURPLE_500;
const ROAD_SECTION_INWARDS_DIRECTION_GIZMO_COLOR: Srgba = TEAL_500;
const DEBUG_CIRCLE_GIZMO_COLOR: Srgba = PURPLE_700;

pub struct RoadDrawerGizmosPlugin;

impl Plugin for RoadDrawerGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_straight_road_section_gizmo,
                draw_road_node_gizmos,
                draw_road_section_end_direction_gizmos,
                draw_curved_road_section_debug_things,
            )
                .in_set(GameRunningSet::DrawGizmos)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn draw_straight_road_section_gizmo(
    mut gizmos: Gizmos,
    road_drawer: Res<RoadDrawer>,
    selected_road: Res<SelectedRoad>,
) {
    if let Some(section_being_drawn) = &road_drawer.section_being_drawn {
        if section_being_drawn.variant != SectionBeingDrawnVariant::Straight {
            return;
        }

        let road_data = selected_road
            .selected_road()
            .expect("A road should always be selected while drawing");

        let road_section_transform = calculate_road_section_gizmo_transform(
            road_data,
            section_being_drawn.start().snapped_position(),
            section_being_drawn.end().snapped_position(),
        );

        gizmos.cuboid(road_section_transform, ROAD_SECTION_GIZMO_COLOR);
    }
}

fn draw_road_node_gizmos(mut gizmos: Gizmos, road_drawer: Res<RoadDrawer>) {
    if let Some(section_being_drawn) = &road_drawer.section_being_drawn {
        for end in section_being_drawn.ends {
            draw_road_node_gizmo(&mut gizmos, end.position, ROAD_NODE_GIZMO_COLOR);
        }
    }
}

fn draw_road_section_end_direction_gizmos(mut gizmos: Gizmos, road_drawer: Res<RoadDrawer>) {
    let Some(section_being_drawn) = &road_drawer.section_being_drawn else {
        return;
    };

    for end in section_being_drawn.ends {
        let Some(direction) = end.direction else {
            continue;
        };

        gizmos.ray(
            end.snapped_position(),
            direction.as_vec3() * 2.0,
            ROAD_SECTION_DIRECTION_GIZMO_COLOR,
        );
    }
}

fn draw_curved_road_section_debug_things(
    mut gizmos: Gizmos,
    road_drawer: Res<RoadDrawer>,
    selected_road: Res<SelectedRoad>,
) {
    if let Some(section_being_drawn) = &road_drawer.section_being_drawn {
        let SectionBeingDrawnVariant::Curved(Some(circular_arc)) = section_being_drawn.variant
        else {
            return;
        };

        let road_data = selected_road
            .selected_road()
            .expect("A road should always be selected while drawing");

        // Cirle center
        gizmos.circle(
            circular_arc.position,
            Dir3::Y,
            0.3,
            DEBUG_CIRCLE_GIZMO_COLOR,
        );

        // Arc line
        gizmos.arc_3d(
            circular_arc.delta_angle,
            circular_arc.radius,
            circular_arc.position,
            Quat::from_axis_angle(Vec3::Y, circular_arc.start_angle + FRAC_PI_2),
            DEBUG_CIRCLE_GIZMO_COLOR,
        );

        // Sections along arc
        for transform_along_arc in circular_arc.calculate_transforms_along_arc(
            ((circular_arc.length().abs() * 1.0).round() as u32).max(5),
            circular_arc.forwards_direction(),
        ) {
            // Center circle
            gizmos.circle(transform_along_arc.translation, Dir3::Y, 0.2, ORANGE_300);

            // Road width lines
            gizmos.ray(
                transform_along_arc.translation,
                transform_along_arc.left() * road_data.half_width(),
                ORANGE_300,
            );

            gizmos.ray(
                transform_along_arc.translation,
                transform_along_arc.right() * road_data.half_width(),
                ORANGE_300,
            );
        }

        // Rays pointing to circle center
        let position = circular_arc.start_position();
        let direction_to_center = circular_arc.rotation_towards_start() * Vec3::Z;

        gizmos.ray(
            position,
            direction_to_center * 100.0,
            ROAD_SECTION_DIRECTION_GIZMO_COLOR,
        );

        let position = circular_arc.end_position();
        let direction_to_center = circular_arc.rotation_towards_end() * Vec3::Z;

        gizmos.ray(
            position,
            direction_to_center * 100.0,
            ROAD_SECTION_DIRECTION_GIZMO_COLOR,
        );

        // Rays pointing inwards from section ends
        let start_transform = circular_arc.outwards_start_transform();

        gizmos.ray(
            start_transform.translation,
            start_transform.back() * 100.0,
            ROAD_SECTION_INWARDS_DIRECTION_GIZMO_COLOR,
        );

        let end_transform = circular_arc.outwards_end_transform();

        gizmos.ray(
            end_transform.translation,
            end_transform.back() * 100.0,
            ROAD_SECTION_INWARDS_DIRECTION_GIZMO_COLOR,
        );

        // Intersection
        if let Some(intersection) = calculate_line_line_intersection_3d(
            Ray3d::new(
                start_transform.translation,
                start_transform.back().as_vec3(),
            ),
            Ray3d::new(end_transform.translation, end_transform.back().as_vec3()),
        ) {
            gizmos.circle(
                intersection,
                Dir3::Y,
                0.3,
                ROAD_SECTION_INWARDS_DIRECTION_GIZMO_COLOR,
            );
        }
    }
}
