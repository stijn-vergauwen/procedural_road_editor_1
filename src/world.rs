use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_objects, spawn_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-6.0, 6.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
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
                base_color: LIME_600.into(),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::splat(2.0))),
            material: materials.add(StandardMaterial {
                base_color: AMBER_500.into(),
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
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
            rotation: Quat::from_rotation_x((-45.0 as f32).to_radians()),
            ..default()
        },
        ..default()
    });
}
