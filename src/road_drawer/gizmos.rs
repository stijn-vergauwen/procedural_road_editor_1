use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    game_modes::GameMode,
    road::{
        road_node::gizmos::draw_road_node_gizmo,
        road_section::gizmos::{draw_curved_road_section_gizmo, draw_straight_road_section_gizmo},
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
                draw_section_being_drawn_gizmo,
                draw_road_node_gizmos,
                draw_road_section_end_direction_gizmos,
                draw_curved_road_section_debug_things,
            )
                .in_set(GameRunningSet::DrawGizmos)
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

fn draw_section_being_drawn_gizmo(
    mut gizmos: Gizmos,
    road_drawer: Res<RoadDrawer>,
    selected_road: Res<SelectedRoad>,
) {
    if let Some(section_being_drawn) = &road_drawer.section_being_drawn {
        let road_design = selected_road
            .selected_road()
            .expect("A road should always be selected while drawing");

        match section_being_drawn.variant {
            SectionBeingDrawnVariant::Straight => draw_straight_road_section_gizmo(
                &mut gizmos,
                road_design,
                section_being_drawn.start().snapped_position(),
                section_being_drawn.end().snapped_position(),
                ROAD_SECTION_GIZMO_COLOR,
            ),
            SectionBeingDrawnVariant::Curved(circular_arc) => {
                let Some(circular_arc) = circular_arc else {
                    return;
                };

                draw_curved_road_section_gizmo(
                    &mut gizmos,
                    road_design,
                    &circular_arc,
                    ROAD_SECTION_GIZMO_COLOR,
                );
            }
        }
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

fn draw_curved_road_section_debug_things(mut gizmos: Gizmos, road_drawer: Res<RoadDrawer>) {
    if let Some(section_being_drawn) = &road_drawer.section_being_drawn {
        let SectionBeingDrawnVariant::Curved(Some(circular_arc)) = section_being_drawn.variant
        else {
            return;
        };

        // Cirle center
        gizmos.circle(
            circular_arc.position,
            Dir3::Y,
            0.3,
            DEBUG_CIRCLE_GIZMO_COLOR,
        );

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
