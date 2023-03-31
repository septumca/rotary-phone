use bevy::{prelude::*};

#[derive(Component)]
pub enum EquippedSkill {
    Fire,
    Slash,
}

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct Character {
  pub attack_cd: Timer,
}

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
pub struct Attack;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Slash;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub enum Controls {
    Idle,
    Move(Vec2),
    Attack(Vec2)
}


#[derive(Component)]
pub struct TTL(pub Timer);

impl TTL {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}