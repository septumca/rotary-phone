use std::f32::consts::{PI};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterController};

use crate::{GameState, components::{TargetPosition, WiggleEffect, RotateAroundPoint, Health, PlayerControlled, EquippedSkill, AttackCD, HealthBar, Character}, SPRITE_DRAW_SIZE};

use super::events::SkillEvent;

pub const WIGGLE_SPEED: f32 = 50.0;
pub const PLAYER_VELOCITY: f32 = 3.0;


pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                input,
                mouse_input,
            )
            .in_set(OnUpdate(GameState::Playing)));
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            move_to_target_position,
            update_wiggle_effect,
            update_rotate_around,
            stop_wiggle_effect,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerInputPlugin)
            .add_plugin(MovementPlugin)
            .add_systems((
                cleanup_on_zero_health,
                update_health_bar,
            ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn cleanup_on_zero_health(
    mut commands: Commands,
    health_q: Query<(Entity, &Health), Changed<Health>>,
) {
    for (entity, health) in health_q.iter() {
        if health.act <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_health_bar(
    health_q: Query<&Health, Changed<Health>>,
    mut healthbar_q: Query<(&Parent, &mut Sprite), (With<HealthBar>, Without<Health>)>,
) {
    for (parent, mut sprite) in healthbar_q.iter_mut() {
        let Ok(health) = health_q.get(parent.get()) else {
            continue;
        };
        let ratio = health.act / health.max;
        sprite.custom_size = Some(Vec2::new(SPRITE_DRAW_SIZE * ratio, 8.0));
    }
}

fn move_to_target_position(
    mut commands: Commands,
    mut movable_q: Query<(Entity, &Character, &TargetPosition, &Transform, &mut KinematicCharacterController)>,
) {
    for (entity, character, target_position, transform, mut controller) in movable_q.iter_mut() {
        let delta_v = target_position.0 - transform.translation.truncate();
        if delta_v.length_squared() < 10.0 {
            commands.entity(entity).remove::<TargetPosition>();
            commands.entity(entity).remove::<WiggleEffect>();
            continue;
        }
        let velocity = delta_v.normalize() * character.speed;
        controller.translation = Some(velocity);
    }
}

fn update_wiggle_effect(
    timer: Res<Time>,
    mut q: Query<(&mut WiggleEffect, &mut Transform)>,
) {
    for (mut wiggle_effect, mut transform) in q.iter_mut() {
        wiggle_effect.act = (wiggle_effect.act + wiggle_effect.speed * timer.delta_seconds()) % (PI * 2.0);
        transform.rotation = Quat::from_rotation_z(wiggle_effect.act.sin() * wiggle_effect.magnitude);
    }
}

fn update_rotate_around(
    timer: Res<Time>,
    mut q: Query<(&RotateAroundPoint, &mut Transform)>,
) {
    let dt = timer.delta_seconds();
    for (rotate_around, mut transform) in q.iter_mut() {
        transform.rotate_around(rotate_around.origin, Quat::from_rotation_z(rotate_around.angvel * dt));
    }
}

fn stop_wiggle_effect(
    mut removals: RemovedComponents<WiggleEffect>,
    mut q: Query<&mut Transform>,
) {
    for entity in removals.iter() {
        let Ok(mut transform) = q.get_mut(entity) else {
            continue;
        };
        transform.rotation = Quat::from_rotation_z(0.0);
    }
}

fn input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(Entity, &Character, &mut KinematicCharacterController, Option<&WiggleEffect>), With<PlayerControlled>>,
) {
    let Ok((entity, character, mut controller, wiggle_effect)) = player_q.get_single_mut() else {
        return;
    };

    let mut velocity = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        velocity.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        velocity.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        velocity.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        velocity.x += 1.0;
    }

    if velocity != Vec2::ZERO {
        controller.translation = Some(velocity.normalize() * character.speed);
        if wiggle_effect.is_none() {
            commands.entity(entity).insert(WiggleEffect::default());
        }
    } else {
        if wiggle_effect.is_some() {
            commands.entity(entity).remove::<WiggleEffect>();
        }
    }
}

fn mouse_input(
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut skill_events: EventWriter<SkillEvent>,
    mut player_q: Query<(Entity, &Transform, &KinematicCharacterController, Option<&EquippedSkill>), (With<PlayerControlled>, Without<Camera>, Without<AttackCD>)>,
) {
    let Ok((entity, transform, controller, equipped_skill)) = player_q.get_single_mut() else {
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

    let Some(equipped_skill) = equipped_skill else {
        return;
    };

    if mouse_button_input.pressed(MouseButton::Left) {
        let spawn_vector = (mouse_position - transform.translation.truncate()).normalize();
        let angle = spawn_vector.y.atan2(spawn_vector.x);
        skill_events.send(SkillEvent {
            kind: equipped_skill.clone(),
            parent: entity,
            angle,
            //magic value so that when player moves forward the attack starts litte bit infront and not otherwise
            start_position: transform.translation.truncate() + controller.translation.unwrap_or(Vec2::ZERO) * 4.0,
            spawn_vector_norm: spawn_vector
        });
    }
}
