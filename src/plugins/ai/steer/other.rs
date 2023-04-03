
use bevy::{prelude::*, math::vec2};
use rand::Rng;

use crate::context_map::AddMethod;

use super::{SteerAi, Behaviour};

#[derive(Component)]
pub struct RandomAroundArea(pub Vec2, pub f32);

impl Behaviour<f32, f32> for RandomAroundArea {
    fn fill_context_map(&self, vector: Vec2, data: f32, ai: &mut SteerAi) {
        ai.chase.add(vector.normalize(), AddMethod::Dot(data));
    }
    fn is_valid(&self, _value: &f32) -> bool {
        true
    }
    fn weight(&self, distance: f32) -> f32 {
        if distance > self.1 {
            1.0
        } else {
            distance/self.1
        }
    }
}

pub fn random_around_area(
    mut ai_q: Query<(&Transform, &mut SteerAi, &RandomAroundArea)>,
) {
    for (
        transform,
        mut ai,
        behavior
    ) in ai_q.iter_mut()  {
        if !ai.update_interval.just_finished() {
            continue;
        }
        let mut rng = rand::thread_rng();
        let center_v = behavior.0 - transform.translation.truncate();
        let center_distance = center_v.length();
        let weight = behavior.weight(center_distance);
        let v = if center_distance > behavior.1 {
            center_v
        } else {
            vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
        };
        behavior.fill_context_map(v.normalize(), weight, &mut ai);
    }
}
