mod rotation;
mod zoom;

use bevy::prelude::*;
use rotation::CameraRotationPlugin;
use zoom::CameraZoomPlugin;

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraRotationPlugin, CameraZoomPlugin))
            .add_systems(Startup, spawn_editor_camera);
    }
}

#[derive(Component)]
pub struct CameraAnchor;

#[derive(Component)]
pub struct EditorCamera;

fn spawn_editor_camera(mut commands: Commands) {
    commands
        .spawn((
            CameraAnchor,
            SpatialBundle {
                transform: Transform::from_rotation(Quat::from_euler(
                    EulerRot::YXZ,
                    0.2,
                    -0.4,
                    0.0,
                )),
                ..default()
            },
        ))
        .with_children(|anchor| {
            anchor.spawn((
                EditorCamera,
                Camera3dBundle {
                    transform: Transform::from_translation(Vec3::Z * 10.0),
                    ..default()
                },
            ));
        });
}
