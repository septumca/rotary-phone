use bevy::{prelude::*};

use crate::{GameState};

use self::{random_walk::update_random_walk_ai, follow::update_follow_ai, steer::SteerAiPlugin};

pub mod follow;
pub mod random_walk;
pub mod steer;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_plugin(SteerAiPlugin)
          .add_systems((
              update_random_walk_ai,
              update_follow_ai,
          ).in_set(OnUpdate(GameState::Playing)));
    }
}


#[derive(Component)]
pub struct RandomWalkAi(pub Timer);


impl RandomWalkAi {
  pub fn new() -> Self {
      Self(Timer::from_seconds(0.0, TimerMode::Once))
  }
}

#[derive(Component)]
pub struct FollowAi {
    pub update_interval: Timer,
    pub target: Entity,
    pub radius: f32
}
