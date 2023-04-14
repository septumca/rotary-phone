use std::f32::consts::FRAC_PI_8;

use bevy::prelude::*;

use crate::plugins::{timers::WithTimer, character::WIGGLE_SPEED};


#[derive(Component)]
pub struct Group(pub usize);

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct Character {
    pub speed: f32
}

#[derive(Component)]
pub struct TargetPosition(pub Vec2);

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

#[derive(Component)]
pub struct Attack {
    pub value: f32
}

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub act: f32,
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct AttackCD(pub Timer);

impl AttackCD {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}

impl WithTimer for AttackCD {
    fn timer(&self) -> &Timer {
        &self.0
    }

    fn timer_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
}

#[derive(Component)]
pub struct TTL(pub Timer);

impl TTL {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}
