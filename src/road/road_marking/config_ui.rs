use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    game_modes::GameMode,
    road::{active_road::ActiveRoad, road_data::RoadData},
    ui::{
        components::{
            inputs::{
                color_input::{ColorInputBuilder, ColorInputConfig},
                labeled_element::LabeledElementBuilder,
                number_input::NumberInputBuilder,
            },
            section::{SectionBuilder, SectionConfig},
            UiComponentBuilder, UiComponentWithChildrenBuilder,
        },
        sidebar::Sidebar,
    },
    GameRunningSet,
};

use super::{
    road_marking_events::{
        hide_road_marking_config::OnHideRoadMarkingConfigRequested,
        show_road_marking_config::OnShowRoadMarkingConfigRequested,
    },
    RoadMarking, RoadMarkingField,
};

pub struct RoadMarkingConfigUiPlugin;

impl Plugin for RoadMarkingConfigUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_show_config_requests, handle_hide_config_requests)
                .in_set(GameRunningSet::HandleCommands)
                .run_if(in_state(GameMode::RoadEditor)),
        );
    }
}

fn handle_show_config_requests(
    mut on_show: EventReader<OnShowRoadMarkingConfigRequested>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    sidebar_query: Query<Entity, With<Sidebar>>,
    active_road: Res<ActiveRoad>,
) {
    for _ in on_show.read() {
        let sidebar = sidebar_query.single();
        let road_markings = active_road.road_data().markings();
        let road_data = active_road.road_data();

        commands
            .entity(sidebar)
            .despawn_descendants()
            .with_children(|sidebar| {
                sidebar
                    .spawn(build_scrollable_list_node())
                    .with_children(|config_container| {
                        for road_marking in road_markings {
                            spawn_road_marking_config(
                                config_container,
                                road_marking,
                                road_data,
                                &mut images,
                            );
                        }
                    });
            });
    }
}

fn spawn_road_marking_config(
    builder: &mut ChildBuilder,
    road_marking: &RoadMarking,
    road_data: &RoadData,
    images: &mut Assets<Image>,
) {
    let x_position_value_range = -road_data.half_width()..road_data.half_width();

    // TODO: make section foldable, so config options are only shown for desired items
    SectionBuilder::new(SectionConfig::default().with_background_color(NEUTRAL_600)).spawn(
        builder,
        (),
        |section| {
            LabeledElementBuilder::centered_top_label("X position").spawn(
                section,
                (),
                |position_label| {
                    NumberInputBuilder::default()
                        .with_values(road_marking.x_position, x_position_value_range)
                        .spawn(position_label, RoadMarkingField::XPosition);
                },
            );

            LabeledElementBuilder::centered_top_label("Segment width").spawn(
                section,
                (),
                |width_label| {
                    NumberInputBuilder::default()
                        .with_values(road_marking.segment_width, 0.0..1.0)
                        .spawn(width_label, RoadMarkingField::SegmentWidth);
                },
            );

            LabeledElementBuilder::centered_top_label("Color").spawn(section, (), |color_label| {
                ColorInputBuilder::new(
                    ColorInputConfig::default().with_start_color(road_marking.color),
                    images,
                )
                .spawn(color_label, RoadMarkingField::Color);
            });
        },
    );
}

fn handle_hide_config_requests(
    mut on_hide: EventReader<OnHideRoadMarkingConfigRequested>,
    mut commands: Commands,
    sidebar_query: Query<Entity, With<Sidebar>>,
) {
    for _ in on_hide.read() {
        let sidebar = sidebar_query.single();

        commands.entity(sidebar).despawn_descendants();
    }
}

// TODO: wanted to make the list scrollable but overflow doesn't seem to work as I thought, delete fn if not gonna use anymore
fn build_scrollable_list_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        ..default()
    }
}
