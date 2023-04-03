use bevy::{prelude::*};

use crate::{components::{Character, Group, Obstacle}, context_map::AddMethod};

use super::{SteerAi, SteerConfiguration, Behaviour};

#[derive(Component)]
pub struct AvoidObstacles(pub SteerConfiguration);

impl Behaviour<f32, f32> for AvoidObstacles {
    fn fill_context_map(&self, vector: Vec2, data: f32, ai: &mut SteerAi) {
        ai.avoid.add(vector.normalize(), AddMethod::Dot(data));
    }
    fn is_valid(&self, value: &f32) -> bool {
        *value < self.0.max_distance
    }
    fn weight(&self, distance: f32) -> f32 {
        if distance < self.0.min_distance {
            1.0
        } else {
            (self.0.max_distance - distance) / self.0.max_distance
        }
    }
}

pub fn avoid_obstacles(
    mut ai_q: Query<(&Transform, &mut SteerAi, &AvoidObstacles)>,
    q: Query<&Transform, With<Obstacle>>,
) {
    for (
        ai_transform,
        mut ai,
        behavior
    ) in ai_q.iter_mut()  {
        if !ai.update_interval.just_finished() {
            continue;
        }
        for transform in q.iter() {
            let v = transform.translation.truncate() - ai_transform.translation.truncate();
            let length = v.length();
            if !behavior.is_valid(&length) {
                continue;
            }

            let v = transform.translation.truncate() - ai_transform.translation.truncate();
            let weight = behavior.weight(length);
            behavior.fill_context_map(v.normalize(), weight, &mut ai);
        }
    }
}

#[derive(Component)]
pub struct AvoidGroup(pub SteerConfiguration, pub usize);

impl Behaviour<(usize, f32), f32> for AvoidGroup {
    fn fill_context_map(&self, vector: Vec2, data: f32, ai: &mut SteerAi) {
        ai.avoid.add(vector.normalize(), AddMethod::Dot(data));
    }
    fn is_valid(&self, value: &(usize, f32)) -> bool {
        value.1 < self.0.max_distance && self.1 == value.0
    }
    fn weight(&self, distance: f32) -> f32 {
        self.0.get_weight(distance)
    }
}

pub fn avoid_group(
    mut ai_q: Query<(Entity, &Transform, &mut SteerAi, &AvoidGroup)>,
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

            let v = transform.translation.truncate() - ai_transform.translation.truncate();
            let weight = behavior.weight(length);
            behavior.fill_context_map(v.normalize(), weight, &mut ai);
        }
    }
}