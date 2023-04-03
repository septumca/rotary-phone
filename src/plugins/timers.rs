use bevy::prelude::*;

use crate::{GameState, components::{TTL, AttackCD}};

pub struct TimersPlugin;

impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                update_cd::<AttackCD>,
                update_ttl,
            ).in_set(OnUpdate(GameState::Playing)));
    }
}

pub trait WithTimer {
    fn timer(&self) -> &Timer;
    fn timer_mut(&mut self) -> &mut Timer;
}

fn update_cd<T>(
    time: Res<Time>,
    mut commands: Commands,
    mut t_q: Query<(Entity, &mut T)>,
)
where T: WithTimer + Component,
{
    let dt = time.delta();
    for (e, mut t) in t_q.iter_mut() {
        if t.timer_mut().tick(dt).finished() {
            commands.entity(e).remove::<T>();
        }
    }
}


fn update_ttl(
    timer: Res<Time>,
    mut commands: Commands,
    mut ttl_q: Query<(Entity, &mut TTL)>,
) {
    let dt = timer.delta();
    for (entity, mut ttl) in ttl_q.iter_mut() {
        if ttl.0.tick(dt).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}