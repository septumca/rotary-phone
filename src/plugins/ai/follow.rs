use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

use crate::components::{Character, Obstacle, WiggleEffect};

use super::FollowAi;

pub fn update_follow_ai(
  mut commands: Commands,
  mut ai_q: Query<(Entity, &Character, &Transform, &FollowAi, &mut KinematicCharacterController, Option<&WiggleEffect>)>,
  wall_q: Query<&Transform, With<Obstacle>>,
) {
  for (
      entity,
      character,
      transform,
      ai,
      mut controller,
      wiggle_effect
  ) in ai_q.iter_mut()  {
      let Ok(target_transform) = wall_q.get(ai.target) else {
          commands.entity(entity).remove::<WiggleEffect>();
          continue;
      };

      let delta_v = target_transform.translation.truncate() - transform.translation.truncate();
      if delta_v.length_squared() < (ai.radius - 10.0).powf(2.0) {
          commands.entity(entity).remove::<WiggleEffect>();
          continue;
      }

      controller.translation = Some(delta_v.normalize() * character.speed);
      if wiggle_effect.is_none() {
          commands.entity(entity).insert(WiggleEffect::default());
      }
  };
}