mod road_builder;

use bevy::prelude::*;
use road_builder::RoadBuilderPlugin;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoadBuilderPlugin)
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

    let active_road = RoadEditor { road };

    commands.insert_resource(active_road);

    on_road_modified.send(OnActiveRoadModified);
}

#[derive(Component)]
pub struct ActiveRoad;

#[derive(Event)]
pub struct OnActiveRoadModified;

#[derive(Resource)]
pub struct RoadEditor {
    road: RoadData,
}

#[derive(Clone)]
pub struct RoadData {
    name: String,
    components: Vec<RoadComponent>,
}

impl RoadData {
    fn total_size(&self) -> Vec2 {
        Vec2::new(self.total_width(), self.total_height())
    }

    fn total_width(&self) -> f32 {
        self.components
            .iter()
            .fold(0.0, |sum, component| sum + component.size.x)
    }

    fn total_height(&self) -> f32 {
        self.components
            .iter()
            .fold(0.0, |sum, component| sum.max(component.size.y))
    }
}

#[derive(Clone)]
pub struct RoadComponent {
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
}
