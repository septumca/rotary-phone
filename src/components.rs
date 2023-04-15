use bevy::prelude::*;

use crate::plugins::timers::WithTimer;


#[derive(Component)]
pub struct Group(pub usize);



#[derive(Component)]
pub struct Attack {
    pub value: f32
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
