use bevy::prelude::*;

use crate::{GameState, components::{Character, TTL}};

pub struct TimersPlugin;

impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                update_character,
                update_ttl,
            ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn update_character(
    timer: Res<Time>,
    mut character_q: Query<&mut Character>,
) {
    for mut ch in character_q.iter_mut() {
        ch.attack_cd.tick(timer.delta());
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