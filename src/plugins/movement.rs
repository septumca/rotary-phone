use std::f32::consts::{FRAC_PI_8};

use bevy::{prelude::*};
use bevy_rapier2d::prelude::KinematicCharacterController;

use crate::{GameState, components::{TargetPosition, WiggleEffect, RotateAroundPoint, Character}};

const WIGGLE_SPEED: f32 = 100.0;
const PLAYER_VELOCITY: f32 = 5.0;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            move_to_target_position,
            movement_detection,
            update_wiggle_effect,
            update_rotate_around,
            stop_wiggle_effect,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn move_to_target_position(
    mut commands: Commands,
    mut movable_q: Query<(Entity, &TargetPosition, &Transform, &mut KinematicCharacterController)>,
) {
    for (entity, target_position, transform, mut controller) in movable_q.iter_mut() {
        let delta_v = target_position.0 - transform.translation.truncate();
        if delta_v.length_squared() < 10.0 {
            commands.entity(entity).remove::<TargetPosition>();
            continue;
        }
        let velocity = delta_v.normalize() * PLAYER_VELOCITY;
        controller.translation = Some(velocity);
    }
}

fn update_wiggle_effect(
    mut q: Query<(&mut WiggleEffect, &mut Transform)>,
) {
    for (mut wiggle_effect, mut transform) in q.iter_mut() {
        if transform.rotation.z.abs() > wiggle_effect.0.abs() {
            wiggle_effect.0 = -wiggle_effect.0;
        }
        transform.rotate_z(WIGGLE_SPEED.sin() * wiggle_effect.0);
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

pub fn movement_detection(
    mut commands: Commands,
    q: Query<(Entity, Option<&TargetPosition>, Option<&WiggleEffect>), With<Character>>,
) {
    for (entity, target_position, wiggle_effect) in q.iter() {
        if target_position.is_some() {
            if wiggle_effect.is_none() {
                commands.entity(entity).insert(WiggleEffect(FRAC_PI_8 / 4.0));
            }
        } else {
            if wiggle_effect.is_some() {
                commands.entity(entity).remove::<WiggleEffect>();
            }
        }
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
