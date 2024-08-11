pub mod mesh_builder;
pub mod texture_builder;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shutdown_on_control_q);
    }
}

// Utility systems

fn shutdown_on_control_q(
    input: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut on_exit: EventWriter<AppExit>,
) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyQ) {
        let Ok(mut window) = window_query.get_single_mut() else {
            return;
        };

        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;

        on_exit.send(AppExit::Success);
    }
}

// Utility functions

pub fn add_rotations_as_eulers(rotation_a: Quat, rotation_b: Quat, euler: EulerRot) -> Quat {
    let euler_a = rotation_a.to_euler(euler);
    let euler_b = rotation_b.to_euler(euler);

    Quat::from_euler(
        euler,
        euler_a.0 + euler_b.0,
        euler_a.1 + euler_b.1,
        euler_a.2 + euler_b.2,
    )
}


pub fn entity_is_descendant_of(parent_query: &Query<&Parent>, entity: Entity, parent: Entity) -> bool {
    parent_query
        .iter_ancestors(entity)
        .find(|ancestor| *ancestor == parent)
        .is_some()
}