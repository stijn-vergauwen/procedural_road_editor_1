use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::GameRunningSet;

use super::components::{section::SectionBuilder, UiComponentWithChildrenBuilder};

pub struct ModalPlugin;

impl Plugin for ModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnShowModalRequested>()
            .add_event::<OnHideModalRequested>()
            .add_systems(Startup, spawn_centered_modal)
            .add_systems(
                Update,
                (
                    (hide_modal_when_clicking_outside).in_set(GameRunningSet::GetUserInput),
                    (handle_hide_requests, handle_show_requests)
                        .chain()
                        .in_set(GameRunningSet::DespawnEntities),
                ),
            );
    }
}

#[derive(Component)]
pub struct Modal;

#[derive(Event)]
pub struct OnShowModalRequested {
    child_entity: Entity,
}

impl OnShowModalRequested {
    pub fn new(child_entity: Entity) -> Self {
        Self { child_entity }
    }
}

#[derive(Event)]
pub struct OnHideModalRequested;

fn spawn_centered_modal(mut commands: Commands, mut on_hide: EventWriter<OnHideModalRequested>) {
    commands
        .spawn(build_centered_container_node())
        .with_children(|container| {
            SectionBuilder::spawn_default(
                container,
                (Modal, RelativeCursorPosition::default()),
                |_| {},
            );
        });

    on_hide.send(OnHideModalRequested);
}

fn handle_show_requests(
    mut requests: EventReader<OnShowModalRequested>,
    mut commands: Commands,
    mut modal_query: Query<(Entity, &mut Visibility), With<Modal>>,
) {
    for request in requests.read() {
        let (modal_entity, mut modal_visibility) = modal_query.single_mut();

        *modal_visibility = Visibility::Inherited;

        commands
            .entity(modal_entity)
            .despawn_descendants()
            .add_child(request.child_entity);
    }
}

fn handle_hide_requests(
    mut requests: EventReader<OnHideModalRequested>,
    mut commands: Commands,
    mut modal_query: Query<(Entity, &mut Visibility), With<Modal>>,
) {
    for _ in requests.read() {
        let (modal_entity, mut modal_visibility) = modal_query.single_mut();

        *modal_visibility = Visibility::Hidden;

        commands.entity(modal_entity).despawn_descendants();
    }
}

fn hide_modal_when_clicking_outside(
    mut on_request: EventWriter<OnHideModalRequested>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    modal_query: Query<&RelativeCursorPosition, With<Modal>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let cursor_position = modal_query.single();

        if !cursor_position.mouse_over() {
            on_request.send(OnHideModalRequested);
        }
    }
}

fn build_centered_container_node() -> impl Bundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}
