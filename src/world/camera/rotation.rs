use std::ops::Range;

use bevy::{ecs::event::EventIterator, input::mouse::MouseMotion, prelude::*};

use crate::{utility::add_rotations_as_eulers, GameRunningSet};

use super::CameraAnchor;

const ROTATION_MULTIPLIER: f32 = 0.0015;
const ACTIVATE_ROTATION_INPUT_BUTTON: MouseButton = MouseButton::Right;
const CAMERA_PITCH_RANGE_DEGREES: Range<f32> = 1.0..80.0;

pub struct CameraRotationPlugin;

impl Plugin for CameraRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnCameraRotationRequested>().add_systems(
            Update,
            (
                listen_to_rotation_input.in_set(GameRunningSet::GetUserInput),
                handle_rotation_requests.in_set(GameRunningSet::HandleCommands),
            ),
        );
    }
}

#[derive(Event)]
struct OnCameraRotationRequested {
    delta_rotation: Quat,
}

impl OnCameraRotationRequested {
    fn new(delta_rotation: Quat) -> Self {
        Self { delta_rotation }
    }
}

fn listen_to_rotation_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut rotation_request: EventWriter<OnCameraRotationRequested>,
) {
    if !has_rotation_input(mouse_motion.is_empty(), &mouse_input) {
        return;
    }

    let mouse_delta = sum_mouse_delta(mouse_motion.read());
    let delta_rotation = rotation_from_mouse_delta(mouse_delta, ROTATION_MULTIPLIER);

    rotation_request.send(OnCameraRotationRequested::new(delta_rotation));
}

fn handle_rotation_requests(
    mut requests: EventReader<OnCameraRotationRequested>,
    mut camera_query: Query<&mut Transform, With<CameraAnchor>>,
) {
    let mut camera_transform = camera_query.single_mut();

    for request in requests.read() {
        let new_rotation = add_rotations_as_eulers(
            camera_transform.rotation,
            request.delta_rotation,
            EulerRot::YXZ,
        );

        let clamped_rotation = clamp_camera_pitch(new_rotation, CAMERA_PITCH_RANGE_DEGREES);

        camera_transform.rotation = clamped_rotation;
    }
}

fn has_rotation_input(mouse_motion_is_empty: bool, mouse_input: &ButtonInput<MouseButton>) -> bool {
    !mouse_motion_is_empty && mouse_input.pressed(ACTIVATE_ROTATION_INPUT_BUTTON)
}

fn sum_mouse_delta(motion_events: EventIterator<MouseMotion>) -> Vec2 {
    motion_events.fold(Vec2::ZERO, |sum, motion| sum + motion.delta)
}

fn rotation_from_mouse_delta(mouse_delta: Vec2, rotation_multiplier: f32) -> Quat {
    let scaled = mouse_delta * rotation_multiplier;
    Quat::from_euler(EulerRot::YXZ, scaled.x, scaled.y, 0.0)
}

fn clamp_camera_pitch(camera_rotation: Quat, pitch_range_degrees: Range<f32>) -> Quat {
    let as_euler = camera_rotation.to_euler(EulerRot::YXZ);
    let min = -pitch_range_degrees.start;
    let max = -pitch_range_degrees.end;

    Quat::from_euler(
        EulerRot::YXZ,
        as_euler.0,
        as_euler.1.clamp(max.to_radians(), min.to_radians()),
        as_euler.2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_rotation_from_mouse_delta() {
        let mouse_delta = Vec2::new(2.0, 3.0);
        let rotation_multiplier = 0.5;
        let expected = Quat::from_euler(
            EulerRot::YXZ,
            mouse_delta.x * rotation_multiplier,
            mouse_delta.y * rotation_multiplier,
            0.0,
        );

        let as_rotation = rotation_from_mouse_delta(mouse_delta, rotation_multiplier);

        assert_eq!(expected, as_rotation);
    }
}
