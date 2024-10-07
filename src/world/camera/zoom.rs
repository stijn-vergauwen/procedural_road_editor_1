use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{game_modes::GameMode, GameRunningSet};

use super::TopDownCamera;

pub struct CameraZoomPlugin;

impl Plugin for CameraZoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnCameraZoomRequested>().add_systems(
            Update,
            (
                listen_to_zoom_input.in_set(GameRunningSet::GetUserInput),
                handle_zoom_requests.in_set(GameRunningSet::HandleCommands),
            )
                .run_if(in_state(GameMode::RoadEditor).or_else(in_state(GameMode::RoadDrawer))),
        );
    }
}

#[derive(Event)]
struct OnCameraZoomRequested {
    delta_zoom: f32,
}

fn listen_to_zoom_input(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut zoom_request: EventWriter<OnCameraZoomRequested>,
    camera_query: Query<&TopDownCamera>,
) {
    let Ok(camera) = camera_query.get_single() else {
        return;
    };

    if !camera.config.zoom.enable_input || camera.config.zoom.zoom_speed <= 0.0 {
        return;
    }

    let delta_scroll = sum_mouse_scroll(mouse_wheel.read());

    if delta_scroll == 0.0 {
        return;
    }

    let delta_zoom = calculate_delta_zoom(delta_scroll, camera.config.zoom.zoom_speed);

    zoom_request.send(OnCameraZoomRequested { delta_zoom });
}

fn handle_zoom_requests(
    mut requests: EventReader<OnCameraZoomRequested>,
    mut camera_query: Query<&mut Transform, With<TopDownCamera>>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    for request in requests.read() {
        camera_transform.translation =
            calculate_new_camera_position(camera_transform.translation, request.delta_zoom);
    }
}

fn sum_mouse_scroll<'a>(wheel_events: impl Iterator<Item = &'a MouseWheel>) -> f32 {
    wheel_events.fold(0.0, |sum, event| sum + event.y)
}

fn calculate_delta_zoom(delta_scroll: f32, multiplier: f32) -> f32 {
    -delta_scroll * multiplier
}

fn calculate_new_camera_position(mut position: Vec3, delta_zoom: f32) -> Vec3 {
    position.z += delta_zoom;

    position
}
