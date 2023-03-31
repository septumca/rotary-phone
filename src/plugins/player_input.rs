use bevy::{prelude::*};

use crate::{
    GameState,
    components::{
        TargetPosition,
        Character,
        Controls,
        PlayerControlled,
        EquippedSkill
    },
};

use super::events::{FireEvent, SlashEvent};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            input.before(apply_controls),
            mouse_input.before(apply_controls),
            apply_controls,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn input(
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::W) {

    }
    if keyboard_input.pressed(KeyCode::S) {

    }
    if keyboard_input.pressed(KeyCode::A) {

    }
    if keyboard_input.pressed(KeyCode::D) {

    }
}

fn apply_controls (
    mut commands: Commands,
    mut player_q: Query<(Entity, &mut Controls, &Transform, Option<&EquippedSkill>), With<PlayerControlled>>,
    mut fire_events: EventWriter<FireEvent>,
    mut slash_events: EventWriter<SlashEvent>,
) {
    let Ok((entity, mut controls, transform, equipped_skill)) = player_q.get_single_mut() else {
        return;
    };

    match *controls {
        Controls::Idle => {},
        Controls::Attack(target) => {
            let Some(equipped_skill) = equipped_skill else {
                return;
            };
            let spawn_vector = (target - transform.translation.truncate()).normalize();
            let angle = spawn_vector.y.atan2(spawn_vector.x);
            match equipped_skill {
                EquippedSkill::Fire => {
                    fire_events.send(FireEvent {
                        angle,
                        start_position: transform.translation.truncate(),
                        spawn_vector_norm: spawn_vector
                    });
                },
                EquippedSkill::Slash => {
                    slash_events.send(SlashEvent {
                        angle,
                        start_position: transform.translation.truncate(),
                        spawn_vector_norm: spawn_vector
                    });
                }
            }
        },
        Controls::Move(target) => {
            commands.entity(entity).insert(TargetPosition(target));
        }
    }
    *controls = Controls::Idle;
}

fn mouse_input(
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<(&mut Character, &mut Controls), (With<PlayerControlled>, Without<Camera>)>
) {
    let Ok((mut character, mut controls)) = player_q.get_single_mut() else {
        return;
    };
    let Ok(window) = window.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };
    let Some(mouse_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate()) else
    {
        return;
    };

    if mouse_button_input.pressed(MouseButton::Left) {
        *controls = Controls::Move(mouse_position);
        return;
    }

    if mouse_button_input.pressed(MouseButton::Right) {
        if !character.attack_cd.finished() {
            return;
        }
        *controls = Controls::Attack(mouse_position);
        character.attack_cd.reset();
        return;
    }
}