use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    road::{
        road_node::{RequestedRoadNode, RoadNode},
        road_section::{
            road_section_builder::OnBuildRoadSectionRequested, RequestedRoadSection,
            RequestedRoadSectionEnd, RoadSectionVariant,
        },
    },
    utility::circular_arc::CircularArc,
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
    pub variant: SectionBeingDrawnVariant,
}

impl SectionBeingDrawn {
    fn to_requested_road_section(&self) -> RequestedRoadSection {
        RequestedRoadSection {
            ends: self.ends.map(|end| end.to_requested_road_section_end()),
            variant: self.variant.to_road_section_variant(),
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
            .expect("Direction should be Some before converting to requested section");

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

    /// Returns this end's direction which faces outwards, or None.
    fn outwards_direction(&self) -> Option<Dir3> {
        self.direction
    }

    /// Returns this end's direction but flipped to face inwards, or None.
    fn inwards_direction(&self) -> Option<Dir3> {
        self.direction.map(|direction| -direction)
    }

    /// Returns a Transform with this end's position facing outwards, or None if no direction is set.
    #[expect(unused)]
    fn outwards_transform(&self) -> Option<Transform> {
        Some(self.get_transform_with_direction(self.outwards_direction()?))
    }

    /// Returns a Transform with this end's position facing inwards, or None if no direction is set.
    fn inwards_transform(&self) -> Option<Transform> {
        Some(self.get_transform_with_direction(self.inwards_direction()?))
    }

    /// Returns a Transform with this end's position and the given direction.
    fn get_transform_with_direction(&self, direction: Dir3) -> Transform {
        Transform::from_translation(self.position).looking_to(direction, Dir3::Y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectionBeingDrawnVariant {
    Straight,
    Curved(Option<CircularArc>),
}

impl SectionBeingDrawnVariant {
    fn to_road_section_variant(&self) -> RoadSectionVariant {
        match self {
            SectionBeingDrawnVariant::Straight => RoadSectionVariant::Straight,
            SectionBeingDrawnVariant::Curved(circular_arc) => RoadSectionVariant::Curved(
                circular_arc
                    .expect("CircularArc should be Some before converting to RoadSectionVariant."),
            ),
        }
    }
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
            variant: SectionBeingDrawnVariant::Curved(None),
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

        let nearest_road_node = find_road_node_nearest_to_point(
            &road_node_query,
            interaction_target.position,
            ROAD_NODE_SNAP_DISTANCE,
        );

        let target_position = match nearest_road_node {
            Some(nearest_node) => nearest_node.position,
            None => interaction_target.position,
        };

        match section_being_drawn.variant {
            SectionBeingDrawnVariant::Straight => {
                let direction = straight_section_end_outwards_direction(
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
            SectionBeingDrawnVariant::Curved(_) => {
                let mut end_direction: Option<Dir3> = None;

                if let Some(inwards_start_transform) =
                    section_being_drawn.start().inwards_transform()
                {
                    let Some(circular_arc) =
                        CircularArc::from_start_transform(inwards_start_transform, target_position)
                    else {
                        continue;
                    };

                    end_direction = Some(circular_arc.outwards_end_transform().forward());
                    section_being_drawn.variant =
                        SectionBeingDrawnVariant::Curved(Some(circular_arc));
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

        if section_being_drawn.variant == SectionBeingDrawnVariant::Curved(None)
            && section_being_drawn.start().direction.is_none()
        {
            let direction = straight_section_end_outwards_direction(
                section_being_drawn.start().position,
                interaction_target.position,
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

fn straight_section_end_outwards_direction(this_end: Vec3, other_end: Vec3) -> Option<Dir3> {
    Dir3::new(this_end - other_end).ok()
}

fn build_section_end(
    interaction_target: InteractionTarget,
    road_node_query: &Query<(Entity, &Transform), With<RoadNode>>,
    direction: Option<Dir3>,
) -> SectionEndBeingDrawn {
    SectionEndBeingDrawn {
        position: interaction_target.position,
        direction,
        nearest_road_node: find_road_node_nearest_to_point(
            road_node_query,
            interaction_target.position,
            ROAD_NODE_SNAP_DISTANCE,
        ),
    }
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
