use bevy::prelude::*;

#[derive(Component)]
pub struct DeleteButton;

#[derive(Event)]
pub struct OnDeleteButtonPressed;

pub fn send_delete_button_pressed_events(
    mut on_pressed: EventWriter<OnDeleteButtonPressed>,
    button_query: Query<&Interaction, (With<DeleteButton>, Changed<Interaction>)>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            on_pressed.send(OnDeleteButtonPressed);
        }
    }
}
