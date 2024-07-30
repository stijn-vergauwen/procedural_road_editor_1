pub mod load;
pub mod reorder_components;
mod road_builder;
pub mod save;

use bevy::prelude::*;
use load::LoadRoadPlugin;
use reorder_components::ReorderRoadComponentsPlugin;
use road_builder::RoadBuilderPlugin;
use save::SaveRoadPlugin;
use serde::{Deserialize, Serialize};

// TODO: make load module, with deserializer & file loader (just use example road for now)

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RoadBuilderPlugin,
            ReorderRoadComponentsPlugin,
            SaveRoadPlugin,
            LoadRoadPlugin,
        ))
        .add_event::<OnActiveRoadModified>()
        .add_systems(Startup, setup_example_road);
    }
}

fn setup_example_road(
    mut commands: Commands,
    mut on_road_modified: EventWriter<OnActiveRoadModified>,
) {
    let road = RoadData {
        name: String::from("Example road"),
        components: vec![
            RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.2)),
            RoadComponent::new("Lane", Vec2::new(4.0, 0.05)),
            RoadComponent::new("Median", Vec2::new(0.4, 0.2)),
            RoadComponent::new("Lane", Vec2::new(4.0, 0.05)),
            RoadComponent::new("Sidewalk", Vec2::new(2.0, 0.2)),
        ],
    };

    let road_editor = RoadEditor { road: road.clone() };

    commands.insert_resource(road_editor);

    on_road_modified.send(OnActiveRoadModified::new(road));
}

#[derive(Component)]
pub struct ActiveRoad;

#[derive(Event)]
pub struct OnActiveRoadModified {
    road: RoadData,
}

impl OnActiveRoadModified {
    pub fn new(road: RoadData) -> Self {
        Self { road }
    }

    pub fn road(&self) -> &RoadData {
        &self.road
    }
}

#[derive(Resource)]
pub struct RoadEditor {
    road: RoadData,
}

impl RoadEditor {
    pub fn road(&self) -> &RoadData {
        &self.road
    }

    pub fn set_road(
        &mut self,
        road: RoadData,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road = road.clone();

        on_road_modified.send(OnActiveRoadModified::new(road));
    }

    pub fn reorder_road_components(
        &mut self,
        component_index: usize,
        requested_component_index: usize,
        on_road_modified: &mut EventWriter<OnActiveRoadModified>,
    ) {
        self.road
            .components
            .swap(component_index, requested_component_index);

        on_road_modified.send(OnActiveRoadModified::new(self.road.clone()));
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoadData {
    #[allow(unused)]
    name: String,
    components: Vec<RoadComponent>,
}

impl RoadData {
    pub fn components(&self) -> &[RoadComponent] {
        &self.components
    }

    #[allow(unused)]
    fn total_size(&self) -> Vec2 {
        Vec2::new(self.total_width(), self.total_height())
    }

    fn total_width(&self) -> f32 {
        self.components
            .iter()
            .fold(0.0, |sum, component| sum + component.size.x)
    }

    #[allow(unused)]
    fn total_height(&self) -> f32 {
        self.components
            .iter()
            .fold(0.0, |sum, component| sum.max(component.size.y))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoadComponent {
    #[allow(unused)]
    name: String,
    size: Vec2,
}

impl RoadComponent {
    pub fn new(name: impl Into<String>, size: Vec2) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }
}
