use bevy::prelude::*;

#[derive(Component)]
pub struct SaveButton;

#[derive(Event)]
pub struct OnSaveButtonPressed;

pub fn send_save_button_pressed_events(
    mut on_pressed: EventWriter<OnSaveButtonPressed>,
    button_query: Query<&Interaction, (With<SaveButton>, Changed<Interaction>)>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            on_pressed.send(OnSaveButtonPressed);
        }
    }
}
