use bevy::prelude::*;

use crate::GameState;
use std::f32::consts::{FRAC_PI_8, PI};
pub const WIGGLE_SPEED: f32 = 50.0;

pub struct WigglePlugin;

impl Plugin for WigglePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            update_wiggle_effect,
            stop_wiggle_effect,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct WiggleEffect {
    pub magnitude: f32,
    pub speed: f32,
    pub act: f32
}

impl WiggleEffect {
    pub fn new(speed: f32, magnitude: f32) -> Self {
        Self { magnitude, speed, act: 0.0 }
    }
}

impl Default for WiggleEffect {
    fn default() -> Self {
        Self::new(WIGGLE_SPEED, FRAC_PI_8 / 4.0)
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

