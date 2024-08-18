mod add_road_component;
pub mod components;
mod load;
mod save;

use add_road_component::AddRoadComponentPlugin;
use bevy::{color::palettes::tailwind::*, prelude::*};
use components::ToolbarComponentsPlugin;
use load::LoadPlugin;
use save::SavePlugin;

use super::{
    components::{
        buttons::{ButtonAction, ButtonBuilder},
        content_wrap::ContentWrapConfig,
        flexbox::{FlexboxBuilder, FlexboxConfig},
        section::{SectionBuilder, SectionConfig},
        text::TextBuilder,
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
            let section_config = SectionConfig {
                wrap: ContentWrapConfig::default().with_background_color(NEUTRAL_300),
                flexbox: FlexboxConfig::row().with_justify(JustifyContent::SpaceBetween),
            };

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
        // TODO: replace with TextButton
        ButtonBuilder::spawn_default(container, ButtonAction::SaveRoad, |save_button| {
            TextBuilder::default_with_text("Save").spawn(save_button, ());
        });

        // TODO: replace with TextButton
        ButtonBuilder::spawn_default(container, ButtonAction::LoadRoad, |save_button| {
            TextBuilder::default_with_text("Load").spawn(save_button, ());
        });
    });
}

fn spawn_add_component_button(builder: &mut ChildBuilder) {
    FlexboxBuilder::new(FlexboxConfig::centered()).spawn(builder, (), |container| {
        // TODO: replace with TextButton
        ButtonBuilder::spawn_default(container, ButtonAction::AddComponent, |save_button| {
            TextBuilder::default_with_text("+").spawn(save_button, ());
        });
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
