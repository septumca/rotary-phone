use bevy::{prelude::*};
use bevy_rapier2d::{prelude::KinematicCharacterController};

use crate::{context_map::{ContextMap, SEGMENT_OFFSETS_CHASE}, GameState, components::{WiggleEffect, Character}};

pub mod avoid;
pub mod chase;
pub mod other;
pub struct SteerAiPlugin;

impl Plugin for SteerAiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                update_ai_timer.before(update_steer_ai),
                avoid::avoid_obstacles.after(update_ai_timer).before(update_steer_ai),
                avoid::avoid_group.after(update_ai_timer).before(update_steer_ai),
                chase::chase_player.after(update_ai_timer).before(update_steer_ai),
                chase::chase_targets.after(update_ai_timer).before(update_steer_ai),
                chase::chase_group.after(update_ai_timer).before(update_steer_ai),
                chase::strafe_around.after(update_ai_timer).before(update_steer_ai),
                other::random_around_area.after(update_ai_timer).before(update_steer_ai),
                update_steer_ai,
            ).in_set(OnUpdate(GameState::Playing)));

        #[cfg(debug_assertions)]
        {
            app.add_systems((
                draw_debug_lines,
            ).in_set(OnUpdate(GameState::Playing)));
        }
    }
}

#[cfg(debug_assertions)]
fn draw_debug_lines() {

}

pub struct SteerConfiguration {
    pub min_distance: f32,
    pub max_distance: f32,
}

impl SteerConfiguration {
    pub fn new(min_distance: f32, max_distance: f32) -> Self {
        Self { min_distance, max_distance }
    }


    pub fn get_weight(&self, distance: f32) -> f32 {
        if distance < self.min_distance {
            1.0
        } else {
            (self.max_distance - distance) / self.max_distance
        }
    }

    pub fn is_in_distance(&self, distance: f32) -> bool {
        distance > self.min_distance && distance < self.max_distance
    }
}


trait Behaviour<V, D> {
    fn weight(&self, distance: f32) -> f32;
    fn fill_context_map(&self, vector: Vec2, data: D, ai: &mut SteerAi);
    fn is_valid(&self, value: &V) -> bool;
  }


#[derive(Component)]
pub struct SteerAi {
    pub update_interval: Timer,
    pub previous: Vec2,
    pub chase: ContextMap,
    pub avoid: ContextMap,
}

impl SteerAi {
    pub fn new(update_interval_seconds: f32) -> Self {
        Self {
            update_interval: Timer::from_seconds(update_interval_seconds, TimerMode::Repeating),
            previous: Vec2::ZERO,
            chase: ContextMap::new(),
            avoid: ContextMap::new()
        }
    }
}

impl Default for SteerAi {
    fn default() -> Self {
        Self::new(0.1)
    }
}

fn update_ai_timer(
    timer: Res<Time>,
    mut ai_q: Query<&mut SteerAi>,
) {
    for mut ai in ai_q.iter_mut()  {
        if ai.update_interval.tick(timer.delta()).just_finished() {
            ai.chase = ContextMap::new();
            ai.avoid = ContextMap::new();
        }
    }
}

pub fn update_steer_ai(
  mut commands: Commands,
  mut ai_q: Query<(Entity, &Character, &mut SteerAi, &mut KinematicCharacterController, Option<&WiggleEffect>)>,
) {
  for (
      entity,
      character,
      mut ai,
      mut controller,
      wiggle_effect
  ) in ai_q.iter_mut()  {
        if !ai.update_interval.just_finished() {
            if ai.previous != Vec2::ZERO {
                controller.translation = Some(ai.previous * character.speed);
                if wiggle_effect.is_none()  {
                    commands.entity(entity).insert(WiggleEffect::default());
                }
            }
            continue;
        }

        let masked = ai.chase.mask(&ai.avoid);
        if masked.is_empty() {
            ai.previous = Vec2::ZERO;
            controller.translation = None;
            commands.entity(entity).remove::<WiggleEffect>();
            continue;
        }

        ai.previous = masked.get_vector(&ai.previous, SEGMENT_OFFSETS_CHASE);
        controller.translation = Some(ai.previous * character.speed);
        if wiggle_effect.is_none()  {
            commands.entity(entity).insert(WiggleEffect::default());
        }
    };
}
