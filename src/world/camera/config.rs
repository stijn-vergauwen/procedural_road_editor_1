use std::ops::Range;

use bevy::prelude::{KeyCode, MouseButton};

#[derive(Clone, Debug)]
pub struct TopDownCameraConfig {
    pub movement: TopDownCameraMovementConfig,
    pub rotation: TopDownCameraRotationConfig,
    pub zoom: TopDownCameraZoomConfig,
    /// The MouseButton the user needs to hold to activate camera movement and rotation.
    pub activate_button: MouseButton,
}

impl Default for TopDownCameraConfig {
    fn default() -> Self {
        Self {
            movement: Default::default(),
            rotation: Default::default(),
            zoom: Default::default(),
            activate_button: MouseButton::Right,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TopDownCameraMovementConfig {
    /// If this camera will react to movement input.
    pub enable_input: bool,

    /// A modifier for how fast the camera anchor moves.
    pub movement_speed: f32,

    pub keybinds: MovementKeybinds,
}

impl Default for TopDownCameraMovementConfig {
    fn default() -> Self {
        Self {
            enable_input: true,
            movement_speed: 1.0,
            keybinds: MovementKeybinds::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TopDownCameraRotationConfig {
    /// If this camera will react to rotation input.
    pub enable_input: bool,

    /// A modifier for how fast this camera turns.
    pub rotation_speed: f32,

    /// The range to clamp the camera's pitch to, in degrees.
    pub pitch_range_degrees: Range<f32>,
}

impl Default for TopDownCameraRotationConfig {
    fn default() -> Self {
        Self {
            enable_input: true,
            rotation_speed: 0.0015,
            pitch_range_degrees: -80.0..-1.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TopDownCameraZoomConfig {
    /// If this camera will react to zoom input.
    pub enable_input: bool,

    /// A modifier for how fast this camera zooms in and out.
    pub zoom_speed: f32,
}

impl Default for TopDownCameraZoomConfig {
    fn default() -> Self {
        Self {
            enable_input: true,
            zoom_speed: 2.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MovementKeybinds {
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

impl Default for MovementKeybinds {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            back: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
        }
    }
}
