use bevy::prelude::*;

use crate::{game_modes::GameMode, GameRunningSet};

use super::{
    config::{MovementKeybinds, TopDownCameraConfig},
    TopDownCamera, TopDownCameraAnchor,
};

pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnCameraMovementRequested>().add_systems(
            Update,
            (
                listen_to_movement_input.in_set(GameRunningSet::GetUserInput),
                handle_movement_requests.in_set(GameRunningSet::HandleCommands),
            )
                .run_if(in_state(GameMode::RoadDrawer)),
        );
    }
}

#[derive(Event)]
struct OnCameraMovementRequested {
    delta_movement: Vec3,
}

impl OnCameraMovementRequested {
    fn new(delta_movement: Vec3) -> Self {
        Self { delta_movement }
    }
}

fn listen_to_movement_input(
    mut on_movement_request: EventWriter<OnCameraMovementRequested>,
    camera_query: Query<&TopDownCamera>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let Ok(camera) = camera_query.get_single() else {
        return;
    };

    let Some(movement_input) =
        calculate_movement_input(&camera.config, &keyboard_input, &mouse_input)
    else {
        return;
    };

    on_movement_request.send(OnCameraMovementRequested::new(movement_input));
}

fn handle_movement_requests(
    mut requests: EventReader<OnCameraMovementRequested>,
    mut camera_anchor_query: Query<&mut Transform, With<TopDownCameraAnchor>>,
) {
    let Ok(mut anchor_transform) = camera_anchor_query.get_single_mut() else {
        return;
    };

    for request in requests.read() {
        anchor_transform.translation =
            calculate_new_anchor_position(&anchor_transform, request.delta_movement);
    }
}

// Movement input calculations

fn calculate_movement_input(
    config: &TopDownCameraConfig,
    keyboard_input: &ButtonInput<KeyCode>,
    mouse_input: &ButtonInput<MouseButton>,
) -> Option<Vec3> {
    if !config.movement.enable_input
        || !mouse_input.pressed(config.activate_button)
        || config.movement.movement_speed <= 0.0
    {
        return None;
    }

    let movement_input = movement_input_as_vector(config.movement.keybinds, keyboard_input);
    if movement_input == Vec3::ZERO {
        return None;
    }

    Some(calculate_delta_movement(
        movement_input,
        config.movement.movement_speed,
    ))
}

fn calculate_delta_movement(movement_input: Vec3, multiplier: f32) -> Vec3 {
    movement_input.normalize_or_zero() * multiplier
}

fn movement_input_as_vector(
    keybinds: MovementKeybinds,
    keyboard_input: &ButtonInput<KeyCode>,
) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(keybinds.forward) {
        direction.z -= 1.0;
    }

    if keyboard_input.pressed(keybinds.back) {
        direction.z += 1.0;
    }

    if keyboard_input.pressed(keybinds.left) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(keybinds.right) {
        direction.x += 1.0;
    }

    direction
}

// Movement calculations

fn calculate_new_anchor_position(anchor_transform: &Transform, delta_movement: Vec3) -> Vec3 {
    let flat_rotation = get_flat_rotation(anchor_transform.rotation);
    anchor_transform.translation + flat_rotation * delta_movement
}

fn get_flat_rotation(rotation: Quat) -> Quat {
    let y_axis_radians = rotation.to_euler(EulerRot::YXZ).0;
    Quat::from_euler(EulerRot::YXZ, y_axis_radians, 0.0, 0.0)
}
