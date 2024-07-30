use bevy::prelude::*;

#[derive(Component)]
pub struct LoadButton;

#[derive(Event)]
pub struct OnLoadButtonPressed;

pub fn send_load_button_pressed_events(
    mut on_pressed: EventWriter<OnLoadButtonPressed>,
    button_query: Query<&Interaction, (With<LoadButton>, Changed<Interaction>)>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            on_pressed.send(OnLoadButtonPressed);
        }
    }
}
