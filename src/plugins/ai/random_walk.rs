use std::time::Duration;

use bevy::{prelude::*, math::vec2};
use rand::Rng;

use crate::components::{TargetPosition, WiggleEffect};

use super::RandomWalkAi;

pub fn update_random_walk_ai(
  mut commands: Commands,
  time: Res<Time>,
  mut ai_q: Query<(Entity, &mut RandomWalkAi, Option<&mut TargetPosition>, Option<&WiggleEffect>)>,
) {
  let mut rng = rand::thread_rng();
  let dt = time.delta();
  for (entity, mut ai, target_positon, wiggle_effect) in ai_q.iter_mut()  {
      if !ai.0.tick(dt).finished() {
          continue;
      }
      let new_position = vec2(rng.gen_range(0_f32..100_f32), rng.gen_range(0_f32..100_f32));
      if let Some(mut tp) = target_positon {
          tp.0 = new_position;
      } else {
          commands.entity(entity).insert(TargetPosition(new_position));
          if wiggle_effect.is_none() {
              commands.entity(entity).insert(WiggleEffect::default());
          }
      }
      ai.0.set_duration(Duration::from_secs_f32(rng.gen_range(0.5_f32..2_f32)));
      ai.0.reset();
  };
}