use bevy::prelude::*;

use crate::{
    game_modes::GameMode,
    ui::components::{
        buttons::TextButtonBuilder,
        content_size::ContentSizeConfig,
        content_wrap::ContentWrapConfig,
        flexbox::FlexboxConfig,
        section::{SectionBuilder, SectionConfig},
        text::TextBuilder,
        UiComponentBuilder, UiComponentWithChildrenBuilder,
    },
    utility::despawn_component_recursive,
};

pub struct SelectedRoadPlugin;

impl Plugin for SelectedRoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMode::RoadDrawer), spawn_selected_road_ui)
            .add_systems(
                OnExit(GameMode::RoadDrawer),
                despawn_component_recursive::<SelectedRoadUi>,
            );
    }
}

#[derive(Component)]
struct SelectedRoadUi;

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
                TextButtonBuilder::default_with_text("Select road").spawn(section, ());

                TextBuilder::default()
                    .with_text("Selected road name")
                    .spawn(section, ());
            });
        });
}
