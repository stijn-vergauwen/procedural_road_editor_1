use bevy::{color::palettes::tailwind::NEUTRAL_200, prelude::*};

use crate::{
    road::{ActiveRoad, RoadComponent},
    ui::ListItem,
    GameRunningSet,
};

use super::RoadComponentItem;

pub struct SelectedRoadComponentPlugin;

impl Plugin for SelectedRoadComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRoadComponentSelected>()
            .add_event::<OnRoadComponentDeselected>()
            .add_systems(
                Update,
                (
                    (
                        send_road_component_selected_events,
                        send_road_component_deselected_events,
                    )
                        .in_set(GameRunningSet::GetUserInput),
                    (
                        update_selected_road_component,
                        deselect_all_road_components_on_event,
                    )
                        .in_set(GameRunningSet::UpdateEntities),
                ),
            );
    }
}

#[derive(Event, Clone)]
pub struct OnRoadComponentSelected {
    component_data: RoadComponent,
    component_item_entity: Entity,
}

impl OnRoadComponentSelected {
    pub fn new(component_data: RoadComponent, component_item_entity: Entity) -> Self {
        Self {
            component_data,
            component_item_entity,
        }
    }
}

#[derive(Event, Clone)]
pub struct OnRoadComponentDeselected;

fn send_road_component_selected_events(
    mut on_selected: EventWriter<OnRoadComponentSelected>,
    road_component_item_query: Query<
        (&ListItem, &RoadComponentItem, &Interaction, Entity),
        Changed<Interaction>,
    >,
    active_road: Res<ActiveRoad>,
) {
    if let Some((list_item, _, _, entity)) =
        road_component_item_query
            .iter()
            .find(|(_, road_component_item, interaction, _)| {
                **interaction == Interaction::Pressed && !road_component_item.is_selected
            })
    {
        let component_data = active_road.road_data().components()[list_item.index()].clone();

        on_selected.send(OnRoadComponentSelected::new(component_data, entity));
    }
}

fn send_road_component_deselected_events(
    mut on_deselected: EventWriter<OnRoadComponentDeselected>,
    road_component_item_query: Query<(&RoadComponentItem, &Interaction), Changed<Interaction>>,
) {
    if road_component_item_query
        .iter()
        .any(|(road_component_item, interaction)| {
            *interaction == Interaction::Pressed && road_component_item.is_selected
        })
    {
        on_deselected.send(OnRoadComponentDeselected);
    }
}

fn update_selected_road_component(
    mut on_selected: EventReader<OnRoadComponentSelected>,
    mut road_component_item_query: Query<(&mut RoadComponentItem, &mut BackgroundColor, Entity)>,
) {
    for event in on_selected.read() {
        println!("Set selected component");

        for (mut item, mut background_color, entity) in road_component_item_query.iter_mut() {
            item.is_selected = event.component_item_entity == entity;

            background_color.0 = if item.is_selected {
                NEUTRAL_200.into()
            } else {
                Color::NONE
            };
        }
    }
}

fn deselect_all_road_components_on_event(
    mut on_deselected: EventReader<OnRoadComponentDeselected>,
    mut road_component_item_query: Query<(&mut RoadComponentItem, &mut BackgroundColor)>,
) {
    for _ in on_deselected.read() {
        println!("Deselect all components");

        for (mut item, mut background_color) in road_component_item_query.iter_mut() {
            item.is_selected = false;

            background_color.0 = Color::NONE;
        }
    }
}
