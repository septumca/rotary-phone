use std::time::Duration;

use bevy::{prelude::*, math::vec2};
use rand::Rng;

use crate::{components::{RandomWalkAi, TargetPosition}, GameState};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            update_ai,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn update_ai(
    mut commands: Commands,
    time: Res<Time>,
    mut ai_q: Query<(Entity, &mut RandomWalkAi, Option<&mut TargetPosition>)>,
) {
    let mut rng = rand::thread_rng();
    let dt = time.delta();
    for (entity, mut ai, target_positon) in ai_q.iter_mut()  {
        if !ai.0.tick(dt).finished() {
            continue;
        }
        let new_position = vec2(rng.gen_range(0_f32..100_f32), rng.gen_range(0_f32..100_f32));
        if let Some(mut tp) = target_positon {
            tp.0 = new_position;
        } else {
            commands.entity(entity).insert(TargetPosition(new_position));
        }
        ai.0.set_duration(Duration::from_secs_f32(rng.gen_range(0.5_f32..2_f32)));
        ai.0.reset();
    };


}