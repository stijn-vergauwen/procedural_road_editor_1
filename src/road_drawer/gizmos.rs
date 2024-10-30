use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    game_modes::GameMode,
    road::{
        road_node::gizmos::draw_road_node_gizmo,
        road_section::{gizmos::calculate_road_section_gizmo_transform, RoadSectionShape},
    },
    GameRunningSet,
};

use super::{selected_road::SelectedRoad, RoadDrawer};

const ROAD_NODE_GIZMO_COLOR: Srgba = EMERALD_300;
const ROAD_SECTION_GIZMO_COLOR: Srgba = SKY_300;
const ROAD_SECTION_DIRECTION_GIZMO_COLOR: Srgba = PURPLE_500;
const DEBUG_CIRCLE_GIZMO_COLOR: Srgba = PURPLE_700;
const DEBUG_CIRCLE_CENTER_LINE_GIZMO_COLOR: Srgba = PURPLE_400;

pub struct RoadDrawerGizmosPlugin;

impl Plugin for RoadDrawerGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_straight_road_section_gizmo,
                draw_road_node_gizmos,
                draw_road_section_end_direction_gizmos,
                draw_road_section_debug_things,
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
        if section_being_drawn.shape != RoadSectionShape::Straight {
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

fn draw_road_section_debug_things(
    mut gizmos: Gizmos,
    road_drawer: Res<RoadDrawer>,
    selected_road: Res<SelectedRoad>,
) {
    if let Some(section_being_drawn) = &road_drawer.section_being_drawn {
        if section_being_drawn.shape != RoadSectionShape::Curved {
            return;
        }

        let road_data = selected_road
            .selected_road()
            .expect("A road should always be selected while drawing");

        for circle in section_being_drawn.debug_circles.iter() {
            // Cirle center
            gizmos.circle(
                circle.position,
                Dir3::Y,
                0.3,
                DEBUG_CIRCLE_GIZMO_COLOR,
            );

            // Center line of road
            gizmos.circle(
                circle.position,
                Dir3::Y,
                circle.radius,
                DEBUG_CIRCLE_CENTER_LINE_GIZMO_COLOR,
            );

            // Inner line of road
            gizmos.circle(
                circle.position,
                Dir3::Y,
                circle.radius - road_data.half_width(),
                DEBUG_CIRCLE_GIZMO_COLOR,
            );

            // Outer line of road
            gizmos.circle(
                circle.position,
                Dir3::Y,
                circle.radius + road_data.half_width(),
                DEBUG_CIRCLE_GIZMO_COLOR,
            );
        }

        for ray in section_being_drawn.debug_rays.iter() {
            let start = Vec3::new(ray.origin.x, 0.0, ray.origin.y);
            let vector = Vec3::new(ray.direction.x, 0.0, ray.direction.y);

            gizmos.ray(start, vector * 100.0, ROAD_SECTION_DIRECTION_GIZMO_COLOR);
        }
    }
}
