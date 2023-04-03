
use bevy::{prelude::*};

use crate::{components::{Character, Group, PlayerControlled}, context_map::AddMethod};

use super::{SteerAi, SteerConfiguration, Behaviour};


#[derive(Component)]
pub struct ChaseGroup(pub SteerConfiguration, pub usize);

impl Behaviour<(usize, f32), f32> for ChaseGroup {
    fn fill_context_map(&self, vector: Vec2, data: f32, ai: &mut SteerAi) {
        ai.chase.add(vector.normalize(), AddMethod::Dot(data));
    }
    fn is_valid(&self, value: &(usize, f32)) -> bool {
        self.0.is_in_distance(value.1) && self.1 == value.0
    }
    fn weight(&self, distance: f32) -> f32 {
        self.0.get_weight(distance)
    }
}

pub fn chase_group(
  mut ai_q: Query<(Entity, &Transform, &mut SteerAi, &ChaseGroup)>,
  q: Query<(Entity, &Group, &Transform), With<Character>>,
) {
    for (
        ai_entity,
        ai_transform,
        mut ai,
        behavior
    ) in ai_q.iter_mut()  {
        if !ai.update_interval.just_finished() {
            continue;
        }
        for (target_entity, group, transform) in q.iter() {
            if target_entity == ai_entity {
                continue;
            }
            let v = transform.translation.truncate() - ai_transform.translation.truncate();
            let length = v.length();
            if !behavior.is_valid(&(group.0, length)) {
                continue;
            }

            let weight = behavior.weight(length);
            behavior.fill_context_map(v.normalize(), weight, &mut ai);
        }
  }
}

#[derive(Component)]
pub struct ChaseTargets(pub SteerConfiguration, pub Vec<Entity>);

impl Behaviour<(Entity, f32), f32> for ChaseTargets {
    fn fill_context_map(&self, vector: Vec2, data: f32, ai: &mut SteerAi) {
        ai.chase.add(vector.normalize(), AddMethod::Dot(data));
    }
    fn is_valid(&self, value: &(Entity, f32)) -> bool {
        self.0.is_in_distance(value.1) && self.1.contains(&value.0)
    }
    fn weight(&self, distance: f32) -> f32 {
        self.0.get_weight(distance)
    }
}

pub fn chase_targets(
    mut ai_q: Query<(&Transform, &mut SteerAi, &ChaseTargets)>,
    q: Query<(Entity, &Transform), With<Character>>,
) {
    for (
        ai_transform,
        mut ai,
        behavior
    ) in ai_q.iter_mut()  {
        if !ai.update_interval.just_finished() {
            continue;
        }
        for (entity, transform) in q.iter() {
            let v = transform.translation.truncate() - ai_transform.translation.truncate();
            let length = v.length();
            if !behavior.is_valid(&(entity, length)) {
                continue;
            }

            let weight = behavior.weight(length);
            behavior.fill_context_map(v.normalize(), weight, &mut ai);
        }
    }
}


#[derive(Component)]
pub struct ChasePlayer(pub SteerConfiguration);

impl Behaviour<f32, f32> for ChasePlayer {
    fn fill_context_map(&self, vector: Vec2, data: f32, ai: &mut SteerAi) {
        ai.chase.add(vector.normalize(), AddMethod::Dot(data));
    }
    fn is_valid(&self, value: &f32) -> bool {
        self.0.is_in_distance(*value)
    }
    fn weight(&self, distance: f32) -> f32 {
        self.0.get_weight(distance)
    }
}

pub fn chase_player(
  mut ai_q: Query<(&Transform, &mut SteerAi, &ChaseTargets)>,
  q: Query<(Entity, &Transform), (With<Character>, With<PlayerControlled>)>,
) {
  for (
      ai_transform,
      mut ai,
      behavior
  ) in ai_q.iter_mut()  {
    if !ai.update_interval.just_finished() {
        continue;
    }
    for (entity, transform) in q.iter() {
        let v = transform.translation.truncate() - ai_transform.translation.truncate();
        let length = v.length();
        if !behavior.is_valid(&(entity, length)) {
            continue;
        }

        let weight = behavior.weight(length);
        behavior.fill_context_map(v.normalize(), weight, &mut ai);
      }
  }
}


#[derive(Component)]
pub struct ChaseThenStrageAround(pub SteerConfiguration, pub Entity);

impl Behaviour<(Entity, f32), (f32, f32)> for ChaseThenStrageAround {
    fn fill_context_map(&self, vector: Vec2, data: (f32, f32), ai: &mut SteerAi) {
        if data.0 > self.0.min_distance {
            ai.chase.add(vector.normalize(), AddMethod::Dot(data.1));
        } else {
            ai.chase.add(vector.normalize(), AddMethod::DotInverted(data.1));
        }
    }
    fn is_valid(&self, value: &(Entity, f32)) -> bool {
        self.1 == value.0 && self.0.max_distance > value.1
    }
    fn weight(&self, distance: f32) -> f32 {
        if distance > self.0.min_distance {
            self.0.get_weight(distance)
        } else {
            (self.0.min_distance - distance) / self.0.min_distance
        }
    }
}

pub fn strafe_around(
    mut ai_q: Query<(&Transform, &mut SteerAi, &ChaseThenStrageAround)>,
    q: Query<(Entity, &Transform)>,
) {
    for (
        ai_transform,
        mut ai,
        behavior
    ) in ai_q.iter_mut()  {
        if !ai.update_interval.just_finished() {
            continue;
        }
        for (entity, transform) in q.iter() {
            let v = transform.translation.truncate() - ai_transform.translation.truncate();
            let length = v.length();
            if !behavior.is_valid(&(entity, length)) {
                continue;
            }
            let weight = behavior.weight(length);
            behavior.fill_context_map(v.normalize(), (length, weight), &mut ai);
        }
    }
}