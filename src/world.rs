mod camera;
pub mod world_interaction;

use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_rapier3d::prelude::*;
use camera::EditorCameraPlugin;
use world_interaction::WorldInteractionPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EditorCameraPlugin, WorldInteractionPlugin))
            .add_systems(Startup, spawn_objects);
    }
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Collider::cuboid(100.0, 0.1, 100.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(200.0, 0.2, 200.0)),
            material: materials.add(StandardMaterial {
                base_color: NEUTRAL_100.into(),
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.1, 0.0),
            ..default()
        },
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_euler(
                EulerRot::YXZ,
                (20.0 as f32).to_radians(),
                (-45.0 as f32).to_radians(),
                0.0,
            ),
            ..default()
        },
        ..default()
    });
}
