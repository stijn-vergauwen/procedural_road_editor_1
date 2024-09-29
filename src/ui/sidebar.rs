mod road_component_config;

use bevy::prelude::*;
use road_component_config::RoadComponentConfigPlugin;

use crate::{game_modes::GameMode, GameRunningSet};

use super::components::{section::SectionBuilder, UiComponentWithChildrenBuilder};

pub struct SidebarPlugin;

impl Plugin for SidebarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoadComponentConfigPlugin).add_systems(
            Update,
            toggle_sidebar_visibility_based_on_content
                .after(GameRunningSet::DespawnEntities)
                .run_if(in_state(GameMode::RoadEditor)),
        );
    }
}

#[derive(Component)]
pub struct Sidebar;

pub fn spawn_sidebar(builder: &mut ChildBuilder) {
    SectionBuilder::spawn_default(builder, Sidebar, |_| {});
}

fn toggle_sidebar_visibility_based_on_content(
    mut sidebar_query: Query<(Option<&Children>, &mut Style), With<Sidebar>>,
) {
    let (children, mut style) = sidebar_query.single_mut();

    let current_display = style.display;
    let target_display = match children {
        Some(_) => Display::Flex,
        None => Display::None,
    };

    if current_display != target_display {
        style.display = target_display;
    }
}
