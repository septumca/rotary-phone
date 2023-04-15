use crate::plugins::character::actor::TargetPosition;
use crate::GameState;
use crate::PlayerControlled;
use bevy::prelude::*;

use super::character::wiggle::WiggleEffect;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                update_follow_ai, 
                update_rush_ai
            ).in_set(OnUpdate(GameState::Playing))
        );
    }
}

#[derive(Component)]
pub struct RushAi {
    timer: Timer,
}

impl RushAi {
    pub fn new(time: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
        }
    }
}

fn update_rush_ai(
    mut commands: Commands,
    time: Res<Time>,
    mut ai_q: Query<(
        Entity,
        &mut RushAi,
        Option<&mut TargetPosition>,
        Option<&WiggleEffect>,
    )>,
    player_q: Query<&Transform, With<PlayerControlled>>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let dt = time.delta();
    for (entity, mut rush_ai, target_position, wiggle_effect) in ai_q.iter_mut() {
        if target_position.is_some() {
            continue;
        }
        if rush_ai.timer.tick(dt).just_finished() {
            rush_ai.timer.reset();
            commands
                .entity(entity)
                .insert(TargetPosition::new(player_transform.translation.truncate()));
            if wiggle_effect.is_none() {
                commands.entity(entity).insert(WiggleEffect::default());
            }
        }
    }
}

#[derive(Component)]
pub struct FollowAi(Timer);

impl FollowAi {
    pub fn new(time: f32) -> Self {
        Self(Timer::from_seconds(time, TimerMode::Repeating))
    }
}

fn update_follow_ai(
    mut commands: Commands,
    time: Res<Time>,
    mut ai_q: Query<(
        Entity,
        &mut FollowAi,
        Option<&mut TargetPosition>,
        Option<&WiggleEffect>,
    )>,
    player_q: Query<&Transform, With<PlayerControlled>>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let dt = time.delta();
    for (entity, mut follow_ai, target_position, wiggle_effect) in ai_q.iter_mut() {
        if !follow_ai.0.tick(dt).just_finished() {
            continue;
        }
        if let Some(mut target_position) = target_position {
            target_position.update(
                player_transform.translation.x,
                player_transform.translation.y,
            );
        } else {
            commands
                .entity(entity)
                .insert(TargetPosition::new(player_transform.translation.truncate()));
        }
        if wiggle_effect.is_none() {
            commands.entity(entity).insert(WiggleEffect::default());
        }
    }
}
