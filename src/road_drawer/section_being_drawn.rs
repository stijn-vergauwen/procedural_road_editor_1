use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    road::{
        road_node::{RequestedRoadNode, RoadNode},
        road_section::{
            road_section_builder::OnBuildRoadSectionRequested, RequestedRoadSection,
            RequestedRoadSectionEnd, RoadSectionShape,
        },
    },
    world::world_interaction::{
        interaction_target::{InteractionTarget, OnWorldInteractionTargetUpdated},
        mouse_interaction_events::{InteractionPhase, OnMouseInteraction},
        WorldInteraction,
    },
    GameRunningSet,
};

use super::{road_drawer_tool::RoadDrawerTool, selected_road::SelectedRoad, RoadDrawer};

const MOUSE_BUTTON_TO_DRAW: MouseButton = MouseButton::Left;
const ROAD_NODE_SNAP_DISTANCE: f32 = 5.0;

pub struct SectionBeingDrawnPlugin;

impl Plugin for SectionBeingDrawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                send_build_section_request_on_mouse_press.in_set(GameRunningSet::SendCommands),
                (
                    start_drawing_road_on_mouse_press,
                    update_road_being_drawn_on_target_update,
                    set_curved_section_direction_on_mouse_press,
                    cancel_road_on_right_click,
                )
                    .chain()
                    .in_set(GameRunningSet::UpdateEntities),
            )
                .run_if(in_state(GameMode::RoadDrawer).and_then(in_state(RoadDrawerTool::Drawer))),
        )
        .add_systems(
            OnExit(RoadDrawerTool::Drawer),
            cancel_road_when_leaving_drawer_tool,
        );
    }
}

#[derive(Clone, Debug)]
pub struct SectionBeingDrawn {
    pub ends: [SectionEndBeingDrawn; 2],
    pub shape: RoadSectionShape,
    pub debug_circles: Vec<DebugCircle>,
    pub debug_rays: Vec<Ray2d>,
}

impl SectionBeingDrawn {
    fn to_requested_road_section(&self) -> RequestedRoadSection {
        RequestedRoadSection {
            ends: self.ends.map(|end| end.to_requested_road_section_end()),
            shape: self.shape,
        }
    }

    pub fn start(&self) -> SectionEndBeingDrawn {
        self.ends[0]
    }

    pub fn end(&self) -> SectionEndBeingDrawn {
        self.ends[1]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SectionEndBeingDrawn {
    /// The position of this end without snapping.
    pub position: Vec3,
    /// The outwards facing direction that this end looks towards.
    pub direction: Option<Dir3>,
    /// The nearest node to snap to.
    pub nearest_road_node: Option<NearestRoadNode>,
}

impl SectionEndBeingDrawn {
    fn to_requested_road_section_end(&self) -> RequestedRoadSectionEnd {
        let direction = self
            .direction
            .expect("Direction should be Some before turing this into requested section");

        let road_node = match self.nearest_road_node {
            Some(nearest_node) => nearest_node.to_requested_road_node(),
            None => RequestedRoadNode::new(self.position, None),
        };

        RequestedRoadSectionEnd {
            road_node,
            direction,
        }
    }

    /// Returns the position of nearest_road_node if Some, otherwise returns the position of self.
    pub fn snapped_position(&self) -> Vec3 {
        match self.nearest_road_node {
            Some(nearest_node) => nearest_node.position,
            None => self.position,
        }
    }
}

/// A helper struct to visualize the circles used to calculate curved sections
#[derive(Clone, Copy, Debug)]
pub struct DebugCircle {
    pub position: Vec3,
    pub radius: f32,
}

impl DebugCircle {
    fn new(position: Vec3, radius: f32) -> Self {
        Self { position, radius }
    }
}

#[derive(Clone, Copy, Debug)]
enum CurveDirection {
    Right,
    Left,
}

// Systems

fn start_drawing_road_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
    road_node_query: Query<(Entity, &Transform), With<RoadNode>>,
    selected_road: Res<SelectedRoad>,
) {
    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Started))
    {
        if !selected_road.has_selected_road() || road_drawer.section_being_drawn.is_some() {
            continue;
        }

        let Some(interaction_target) = world_interaction.interaction_target() else {
            continue;
        };

        let road_section_end = build_section_end(interaction_target, &road_node_query, None);

        let section_being_drawn = SectionBeingDrawn {
            ends: [road_section_end; 2],
            shape: RoadSectionShape::Curved,
            debug_circles: Vec::new(),
            debug_rays: Vec::new(),
        };

        road_drawer.section_being_drawn = Some(section_being_drawn);
    }
}

fn update_road_being_drawn_on_target_update(
    mut on_target_updated: EventReader<OnWorldInteractionTargetUpdated>,
    mut road_drawer: ResMut<RoadDrawer>,
    road_node_query: Query<(Entity, &Transform), With<RoadNode>>,
) {
    for interaction_target in on_target_updated
        .read()
        .filter_map(|event| event.interaction_target)
    {
        let Some(section_being_drawn) = &mut road_drawer.section_being_drawn else {
            continue;
        };

        match section_being_drawn.shape {
            RoadSectionShape::Straight => {
                let direction = calculate_section_end_direction(
                    section_being_drawn.end().snapped_position(),
                    section_being_drawn.start().snapped_position(),
                );

                section_being_drawn.ends[1] =
                    build_section_end(interaction_target, &road_node_query, direction);

                section_being_drawn.ends[0].direction = section_being_drawn
                    .end()
                    .direction
                    .map(|direction| -direction);
            }
            RoadSectionShape::Curved => {
                // TODO: refactor this block

                let mut end_direction: Option<Dir3> = None;

                if let Some(start_direction) = section_being_drawn.start().direction {
                    section_being_drawn.debug_circles.clear();
                    section_being_drawn.debug_rays.clear();

                    // Steps completed!
                    //  * calculate angle between inverted start_direction & direction from start_position to end_position
                    //  * double this angle to get the end_direction
                    //  * calculate vectors that are perpendicular to start & end directions that are pointing inwards
                    //  * calculate the intersection of these inwards pointing perpendiculars
                    //  * make debug circles for the inside and outside of the road width

                    // TODO: move to SectionEndBeingDrawn, for normal & inverted transforms
                    let inverted_start_transform =
                        Transform::from_translation(section_being_drawn.start().position)
                            .looking_to(-start_direction, Dir3::Y);

                    let transform_looking_at_target =
                        inverted_start_transform.looking_at(interaction_target.point, Dir3::Y);

                    let delta_rotation = transform_looking_at_target.rotation
                        * inverted_start_transform.rotation.inverse();

                    let end_transform = Transform::from_translation(interaction_target.point)
                        .with_rotation(
                            inverted_start_transform.rotation * delta_rotation * delta_rotation,
                        );

                    end_direction = Some(end_transform.forward());

                    // Debug prints

                    // let direction_from_start_to_end = transform_looking_at_target.forward();

                    // let dot_test = (-start_direction).dot(direction_from_start_to_end.as_vec3());
                    // println!("Dot test: {:?}", dot_test);

                    let angle_test = delta_rotation.to_euler(EulerRot::YXZ).0;
                    let curve_direction = if angle_test.is_sign_positive() {
                        CurveDirection::Left
                    } else {
                        CurveDirection::Right
                    };
                    // println!("Road curves to the {:?}", curve_direction);
                    // println!("Angle test: {}", angle_test);

                    // End of debug prints

                    let inwards_direction_from_start = match curve_direction {
                        CurveDirection::Right => inverted_start_transform.right(),
                        CurveDirection::Left => inverted_start_transform.left(),
                    };

                    let inwards_direction_from_end = match curve_direction {
                        CurveDirection::Right => end_transform.right(),
                        CurveDirection::Left => end_transform.left(),
                    };

                    let inwards_ray_from_start = Ray2d::new(
                        inverted_start_transform.translation.xz(),
                        inwards_direction_from_start.xz(),
                    );

                    let inwards_ray_from_end = Ray2d::new(
                        end_transform.translation.xz(),
                        inwards_direction_from_end.xz(),
                    );

                    section_being_drawn.debug_rays.push(inwards_ray_from_start);
                    section_being_drawn.debug_rays.push(inwards_ray_from_end);

                    let intersection_point_2d = calculate_line_line_intersection(
                        inwards_ray_from_start,
                        inwards_ray_from_end,
                    )
                    .expect("Lines should never be parallel");

                    let intersection_point =
                        Vec3::new(intersection_point_2d.x, 0.0, intersection_point_2d.y);

                    let curve_radius = section_being_drawn
                        .start()
                        .position
                        .distance(intersection_point);

                    section_being_drawn
                        .debug_circles
                        .push(DebugCircle::new(intersection_point, curve_radius));

                    /// Chat-gpt generated implementation to calculate intersection point from 2 lines
                    fn calculate_line_line_intersection(
                        line1: Ray2d,
                        line2: Ray2d,
                    ) -> Option<Vec2> {
                        // Calculate the denominator of the intersection formula
                        let denominator = line1.direction.perp_dot(line2.direction.as_vec2());

                        if denominator.abs() < f32::EPSILON {
                            // Lines are parallel or coincident if the denominator is 0
                            return None;
                        }

                        // Calculate the difference vector between the points
                        let delta_origin = line2.origin - line1.origin;

                        // Calculate the intersection scalar `t` for the first line
                        let t = delta_origin.perp_dot(line2.direction.as_vec2()) / denominator;

                        // Use `t` to find the intersection point along the first line
                        Some(line1.origin + line1.direction * t)
                    }
                }

                section_being_drawn.ends[1] =
                    build_section_end(interaction_target, &road_node_query, end_direction);
            }
        }
    }
}

fn set_curved_section_direction_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
    world_interaction: Res<WorldInteraction>,
) {
    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Started))
    {
        let Some(section_being_drawn) = &mut road_drawer.section_being_drawn else {
            continue;
        };

        let Some(interaction_target) = world_interaction.interaction_target() else {
            continue;
        };

        if section_being_drawn.shape == RoadSectionShape::Curved
            && section_being_drawn.start().direction.is_none()
        {
            let direction = calculate_section_end_direction(
                section_being_drawn.start().position,
                interaction_target.point,
            );

            section_being_drawn.ends[0].direction = direction;
        }
    }
}

fn send_build_section_request_on_mouse_press(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut on_request_section: EventWriter<OnBuildRoadSectionRequested>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    for _ in on_interaction
        .read()
        .filter(|event| filter_mouse_interaction(event, InteractionPhase::Started))
    {
        let Some(section_being_drawn) = &road_drawer.section_being_drawn else {
            continue;
        };

        if section_being_drawn.start().direction.is_none()
            || section_being_drawn.end().direction.is_none()
        {
            continue;
        }

        on_request_section.send(OnBuildRoadSectionRequested::new(
            section_being_drawn.to_requested_road_section(),
        ));
        road_drawer.section_being_drawn = None;
    }
}

fn cancel_road_on_right_click(
    mut on_interaction: EventReader<OnMouseInteraction>,
    mut road_drawer: ResMut<RoadDrawer>,
) {
    for _ in on_interaction.read().filter(|event| {
        event.button == MouseButton::Right && event.phase == InteractionPhase::Started
    }) {
        road_drawer.section_being_drawn = None;
    }
}

fn cancel_road_when_leaving_drawer_tool(mut road_drawer: ResMut<RoadDrawer>) {
    road_drawer.section_being_drawn = None;
}

// Utility

fn filter_mouse_interaction(event: &&OnMouseInteraction, phase: InteractionPhase) -> bool {
    event.button == MOUSE_BUTTON_TO_DRAW && event.phase == phase && !event.is_on_ui
}

fn calculate_section_end_direction(position: Vec3, other_position: Vec3) -> Option<Dir3> {
    Dir3::new(position - other_position).ok()
}

// Snapping utility

#[derive(Clone, Copy, Debug)]
pub struct NearestRoadNode {
    position: Vec3,
    entity: Entity,
}

impl NearestRoadNode {
    fn to_requested_road_node(&self) -> RequestedRoadNode {
        RequestedRoadNode::new(self.position, Some(self.entity))
    }
}

fn build_section_end(
    interaction_target: InteractionTarget,
    road_node_query: &Query<(Entity, &Transform), With<RoadNode>>,
    direction: Option<Dir3>,
) -> SectionEndBeingDrawn {
    SectionEndBeingDrawn {
        position: interaction_target.point,
        direction,
        nearest_road_node: find_road_node_nearest_to_point(
            road_node_query,
            interaction_target.point,
            ROAD_NODE_SNAP_DISTANCE,
        ),
    }
}

fn find_road_node_nearest_to_point(
    road_node_query: &Query<(Entity, &Transform), With<RoadNode>>,
    point: Vec3,
    max_distance: f32,
) -> Option<NearestRoadNode> {
    let (node_entity, node_transform, node_distance) = road_node_query
        .iter()
        .map(|(node_entity, node_transform)| {
            (
                node_entity,
                node_transform,
                node_transform.translation.distance(point),
            )
        })
        .min_by(|(_, _, distance_a), (_, _, distance_b)| distance_a.total_cmp(&distance_b))?;

    (node_distance < max_distance).then_some(NearestRoadNode {
        position: node_transform.translation,
        entity: node_entity,
    })
}
