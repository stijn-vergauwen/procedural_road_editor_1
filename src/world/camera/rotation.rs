use std::ops::Range;

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{game_modes::GameMode, utility::add_rotations_as_eulers, GameRunningSet};

use super::CameraAnchor;

const ROTATION_MULTIPLIER: f32 = 0.0015;
const ACTIVATE_ROTATION_INPUT_BUTTON: MouseButton = MouseButton::Right;
const CAMERA_PITCH_RANGE_DEGREES: Range<f32> = -80.0..-1.0;

pub struct CameraRotationPlugin;

impl Plugin for CameraRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnCameraRotationRequested>().add_systems(
            Update,
            (
                listen_to_rotation_input.in_set(GameRunningSet::GetUserInput),
                handle_rotation_requests.in_set(GameRunningSet::HandleCommands),
                grab_cursor_on_rotation_input.in_set(GameRunningSet::UpdateEntities),
            )
                .run_if(in_state(GameMode::RoadEditor).or_else(in_state(GameMode::RoadDrawer))),
        );
    }
}

#[derive(Event)]
struct OnCameraRotationRequested {
    delta_rotation: Quat,
}

fn grab_cursor_on_rotation_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if mouse_input.just_pressed(ACTIVATE_ROTATION_INPUT_BUTTON) {
        let mut window = window_query.single_mut();

        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if mouse_input.just_released(ACTIVATE_ROTATION_INPUT_BUTTON) {
        let mut window = window_query.single_mut();

        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
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

    rotation_request.send(OnCameraRotationRequested { delta_rotation });
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

fn sum_mouse_delta<'a>(motion_events: impl Iterator<Item = &'a MouseMotion>) -> Vec2 {
    motion_events.fold(Vec2::ZERO, |sum, motion| sum + motion.delta)
}

fn rotation_from_mouse_delta(mouse_delta: Vec2, rotation_multiplier: f32) -> Quat {
    let scaled = mouse_delta * rotation_multiplier;
    Quat::from_euler(EulerRot::YXZ, scaled.x, scaled.y, 0.0)
}

fn clamp_camera_pitch(camera_rotation: Quat, pitch_range_degrees: Range<f32>) -> Quat {
    let as_euler = camera_rotation.to_euler(EulerRot::YXZ);
    let min = pitch_range_degrees.start.to_radians();
    let max = pitch_range_degrees.end.to_radians();

    let clamped_pitch = as_euler.1.clamp(min, max);

    Quat::from_euler(EulerRot::YXZ, as_euler.0, clamped_pitch, as_euler.2).normalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_sum_mouse_delta() {
        let motion_events = vec![
            MouseMotion {
                delta: Vec2::new(0.2, 1.0),
            },
            MouseMotion {
                delta: Vec2::new(0.1, -0.4),
            },
        ];

        let summed = sum_mouse_delta(motion_events.iter());

        assert_eq!(Vec2::new(0.3, 0.6), summed);
    }

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

    #[test]
    fn can_clamp_camera_pitch() {
        let range = -60.0..-20.0;

        let rotation_below_clamp = build_quat(-30.0, -80.0);
        let rotation_in_clamp = build_quat(0.0, -50.0);
        let rotation_above_clamp = build_quat(70.0, -10.0);

        let clamped_below = clamp_camera_pitch(rotation_below_clamp, range.clone());
        let clamped_in = clamp_camera_pitch(rotation_in_clamp, range.clone());
        let clamped_above = clamp_camera_pitch(rotation_above_clamp, range.clone());

        assert_eq!(0.0, build_quat(-30.0, -60.0).angle_between(clamped_below));
        assert_eq!(0.0, build_quat(0.0, -50.0).angle_between(clamped_in));
        assert_eq!(0.0, build_quat(70.0, -20.0).angle_between(clamped_above));

        fn build_quat(yaw_deg: f32, pitch_deg: f32) -> Quat {
            Quat::from_euler(
                EulerRot::YXZ,
                yaw_deg.to_radians(),
                pitch_deg.to_radians(),
                0.0,
            )
        }
    }
}
