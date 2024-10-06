use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    ui::components::{
        buttons::{ButtonAction, TextButtonBuilder},
        content_size::ContentSizeConfig,
        content_wrap::ContentWrapConfig,
        flexbox::FlexboxConfig,
        section::{SectionBuilder, SectionConfig},
        text::TextBuilder,
        UiComponentBuilder, UiComponentWithChildrenBuilder,
    },
    utility::despawn_component_recursive,
    GameRunningSet,
};

use super::events::OnRoadSelected;

pub struct SelectedRoadUiPlugin;

impl Plugin for SelectedRoadUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMode::RoadDrawer), spawn_selected_road_ui)
            .add_systems(
                OnExit(GameMode::RoadDrawer),
                despawn_component_recursive::<SelectedRoadUi>,
            )
            .add_systems(
                Update,
                update_selected_road_name_on_road_selected
                    .in_set(GameRunningSet::UpdateEntities)
                    .run_if(in_state(GameMode::RoadDrawer)),
            );
    }
}

#[derive(Component)]
struct SelectedRoadUi;

#[derive(Component)]
struct SelectedRoadName;

fn spawn_selected_road_ui(mut commands: Commands) {
    let selected_road_container_node = SectionBuilder::new(SectionConfig {
        wrap: ContentWrapConfig::empty(),
        flexbox: FlexboxConfig::horizontally_centered_column().with_justify(JustifyContent::End),
        size: ContentSizeConfig::full(),
    })
    .build();

    commands
        .spawn((selected_road_container_node, SelectedRoadUi))
        .with_children(|container| {
            SectionBuilder::new(
                SectionConfig::default()
                    .with_flexbox_config(FlexboxConfig::row().with_px_gap(20.0)),
            )
            .spawn(container, (), |section| {
                TextButtonBuilder::default_with_text("Select road")
                    .spawn(section, ButtonAction::LoadRoad);

                TextBuilder::default()
                    .with_text("Selected road name")
                    .spawn(section, SelectedRoadName);
            });
        });
}

fn update_selected_road_name_on_road_selected(
    mut on_selected: EventReader<OnRoadSelected>,
    mut selected_road_name_query: Query<&mut Text, With<SelectedRoadName>>,
) {
    for event in on_selected.read() {
        let Ok(mut selected_road_name) = selected_road_name_query.get_single_mut() else {
            continue;
        };

        selected_road_name.sections[0].value = event.road_data.name().to_string();
    }
}
