use bevy::prelude::*;

use crate::GameState;

const FLIP_INTERVAL: f32 = 0.1;

#[derive(Component)]
pub struct FlipEffect(Timer);

impl FlipEffect {
    pub fn new() -> Self {
        Self(Timer::from_seconds(FLIP_INTERVAL, TimerMode::Repeating))
    }
}

pub struct FlipEffectPlugin;


impl Plugin for FlipEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                update_flip_effect,
            ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn update_flip_effect(
    time: Res<Time>,
    mut q: Query<(&mut Sprite, &mut FlipEffect)>,
) {
    let dt = time.delta();
    for (mut sprite, mut flip_effect) in q.iter_mut() {
        if flip_effect.0.tick(dt).just_finished() {
            sprite.flip_x = !sprite.flip_x;
        }
    }
}
