use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow, WindowFocused},
};

pub struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shutdown_on_esc, toggle_cursor_on_focus));
    }
}

// Helper systems

fn shutdown_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut on_exit: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        let Ok(mut window) = window_query.get_single_mut() else {
            return;
        };

        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;

        on_exit.send(AppExit::Success);
    }
}

fn toggle_cursor_on_focus(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut on_focused: EventReader<WindowFocused>,
) {
    for event in on_focused.read() {
        let Ok(mut window) = window_query.get_mut(event.window) else {
            return;
        };

        let cursor_grab_mode = match event.focused {
            true => CursorGrabMode::Confined,
            false => CursorGrabMode::None,
        };

        window.cursor.visible = !event.focused;
        window.cursor.grab_mode = cursor_grab_mode;
    }
}

// Helper functions

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
