use bevy::{prelude::*};

use crate::{plugins::timers::WithTimer};

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct RandomWalkAi(pub Timer);

impl RandomWalkAi {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.0, TimerMode::Once))
    }
}

#[derive(Component, Clone)]
pub enum EquippedSkill {
    FireBall,
    Punch(f32),
    Slash,
}

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct TargetPosition(pub Vec2);

#[derive(Component)]
pub struct WiggleEffect(pub f32);

#[derive(Component)]
pub struct RotateAroundPoint {
    pub origin: Vec3,
    pub angvel: f32,
}

impl RotateAroundPoint {
    pub fn new(origin: Vec3, angvel: f32) -> Self {
        Self { origin, angvel }
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
pub struct Projectile;

#[derive(Component)]
pub struct Slash;

#[derive(Component)]
pub struct Wall;

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
