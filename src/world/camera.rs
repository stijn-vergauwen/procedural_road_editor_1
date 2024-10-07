mod config;
mod rotation;
mod zoom;

use bevy::prelude::*;
use config::TopDownCameraConfig;
use rotation::CameraRotationPlugin;
use zoom::CameraZoomPlugin;

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraRotationPlugin, CameraZoomPlugin))
            .add_systems(Startup, spawn_editor_camera);
    }
}

// TODO: reset camera position, rotation and zoom when entering editor & drawer

#[derive(Component)]
pub struct TopDownCamera {
    anchor_entity: Entity,
    config: TopDownCameraConfig,
}

#[derive(Component)]
pub struct TopDownCameraAnchor;

fn spawn_editor_camera(mut commands: Commands) {
    let camera_config = TopDownCameraConfig::default();

    let mut anchor_commands = commands.spawn((
        TopDownCameraAnchor,
        SpatialBundle {
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.2, -0.4, 0.0)),
            ..default()
        },
    ));
    let anchor_entity = anchor_commands.id();

    anchor_commands.with_children(|anchor| {
        anchor.spawn((
            TopDownCamera {
                anchor_entity,
                config: camera_config,
            },
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::Z * 10.0),
                ..default()
            },
        ));
    });
}
