mod add_road_component;
pub mod components;
mod load;
mod save;

use add_road_component::AddRoadComponentPlugin;
use bevy::prelude::*;
use components::ToolbarComponentsPlugin;
use load::LoadPlugin;
use save::SavePlugin;

use super::{
    components::{
        buttons::{ButtonAction, TextButtonBuilder},
        flexbox::{FlexboxBuilder, FlexboxConfig},
        section::{SectionBuilder, SectionConfig},
        UiComponentBuilder, UiComponentWithChildrenBuilder,
    },
    list::List,
};

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ToolbarComponentsPlugin,
            SavePlugin,
            LoadPlugin,
            AddRoadComponentPlugin,
        ))
        .add_systems(Startup, spawn_toolbar);
    }
}

#[derive(Component)]
struct Toolbar;

#[derive(Component)]
struct RoadComponentsList;

pub fn spawn_toolbar(mut commands: Commands) {
    commands
        .spawn(build_container_node())
        .with_children(|container| {
            let flexbox_config = FlexboxConfig::row().with_justify(JustifyContent::SpaceBetween);
            let section_config = SectionConfig::default().with_flexbox_config(flexbox_config);

            SectionBuilder::new(section_config).spawn(container, Toolbar, |toolbar| {
                spawn_action_buttons(toolbar);

                FlexboxBuilder::new(
                    FlexboxConfig::horizontally_centered_row()
                        .with_align(AlignItems::End)
                        .with_px_gap(16.0),
                )
                .spawn(toolbar, (RoadComponentsList, List), |_| {});

                spawn_add_component_button(toolbar);
            });
        });
}

fn spawn_action_buttons(builder: &mut ChildBuilder) {
    FlexboxBuilder::new(FlexboxConfig::column().with_px_gap(4.0)).spawn(builder, (), |container| {
        TextButtonBuilder::default_with_text("Save").spawn(container, ButtonAction::SaveRoad);
        TextButtonBuilder::default_with_text("Load").spawn(container, ButtonAction::LoadRoad);
    });
}

fn spawn_add_component_button(builder: &mut ChildBuilder) {
    FlexboxBuilder::new(FlexboxConfig::centered()).spawn(builder, (), |container| {
        TextButtonBuilder::default_with_text("+").spawn(container, ButtonAction::AddComponent);
    });
}

// TODO: the toolbar shouldn't start at the root level, toolbar and sidebar overlap because of this

fn build_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Stretch,
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        ..default()
    }
}
