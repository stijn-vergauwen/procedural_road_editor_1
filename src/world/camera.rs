mod config;
mod movement;
mod rotation;
mod zoom;

use bevy::prelude::*;
use config::TopDownCameraConfig;
use movement::CameraMovementPlugin;
use rotation::CameraRotationPlugin;
use zoom::CameraZoomPlugin;

use crate::game_modes::GameMode;

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraRotationPlugin, CameraZoomPlugin, CameraMovementPlugin))
            .add_systems(
                OnEnter(GameMode::RoadEditor),
                reset_camera_transforms_to_editor_defaults,
            )
            .add_systems(
                OnEnter(GameMode::RoadDrawer),
                reset_camera_transforms_to_drawer_defaults,
            )
            .add_systems(Startup, spawn_top_down_camera);
    }
}

#[derive(Component)]
pub struct TopDownCamera {
    pub config: TopDownCameraConfig,
}

#[derive(Component)]
pub struct TopDownCameraAnchor;

fn spawn_top_down_camera(mut commands: Commands) {
    let camera_config = TopDownCameraConfig::default();

    commands
        .spawn((TopDownCameraAnchor, SpatialBundle::default()))
        .with_children(|anchor| {
            anchor.spawn((
                TopDownCamera {
                    config: camera_config,
                },
                Camera3dBundle::default(),
            ));
        });
}

fn reset_camera_transforms_to_editor_defaults(
    mut camera_query: Query<&mut Transform, (With<TopDownCamera>, Without<TopDownCameraAnchor>)>,
    mut anchor_query: Query<&mut Transform, With<TopDownCameraAnchor>>,
) {
    set_camera_transforms(
        camera_query.iter_mut(),
        anchor_query.iter_mut(),
        CameraTransforms::editor_defaults(),
    );
}

fn reset_camera_transforms_to_drawer_defaults(
    mut camera_query: Query<&mut Transform, (With<TopDownCamera>, Without<TopDownCameraAnchor>)>,
    mut anchor_query: Query<&mut Transform, With<TopDownCameraAnchor>>,
) {
    set_camera_transforms(
        camera_query.iter_mut(),
        anchor_query.iter_mut(),
        CameraTransforms::drawer_defaults(),
    );
}

fn set_camera_transforms<'a>(
    cameras: impl Iterator<Item = Mut<'a, Transform>>,
    anchors: impl Iterator<Item = Mut<'a, Transform>>,
    camera_transforms: CameraTransforms,
) {
    for mut transform in cameras {
        *transform = camera_transforms.camera;
    }

    for mut transform in anchors {
        *transform = camera_transforms.anchor;
    }
}

#[derive(Clone, Copy)]
pub struct CameraTransforms {
    anchor: Transform,
    camera: Transform,
}

impl CameraTransforms {
    fn editor_defaults() -> Self {
        Self {
            anchor: Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, -0.3, -0.3, 0.0)),
            camera: Transform::from_translation(Vec3::Z * 30.0),
        }
    }

    fn drawer_defaults() -> Self {
        Self {
            anchor: Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, -0.5, 0.0)),
            camera: Transform::from_translation(Vec3::Z * 50.0),
        }
    }
}
