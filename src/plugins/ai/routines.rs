use crate::plugins::character::Character;
use crate::plugins::character::actor::TargetPosition;
use crate::{GameState, PlayerControlled};
use bevy::prelude::*;

use super::events::{EventDistanceReached, EventDistanceExited};

pub struct RoutinesPlugin;

impl Plugin for RoutinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (update_speed, update_follow_routine, update_rush_routine).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

const RUSH_SPEED_BONUS: f32 = 2.0;

#[derive(Component)]
pub struct RushRoutine {
    timer: Timer,
    distance: f32,
}

impl RushRoutine {
    pub fn new(time: f32, distance: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
            distance,
        }
    }
}

fn update_speed(
    mut removed: RemovedComponents<RushRoutine>,
    mut added_q: Query<&mut Character, Added<RushRoutine>>,
    mut removed_q: Query<&mut Character, Without<RushRoutine>>,
) {
    for re in removed.iter() {
        let Ok(mut character) = removed_q.get_mut(re) else {
            continue;
        };
        info!("removing speed bonus");
        character.update_speed(-RUSH_SPEED_BONUS);
    }

    for mut character in added_q.iter_mut() {
        info!("adding speed bonus");
        character.update_speed(RUSH_SPEED_BONUS);
    }
}

fn update_rush_routine(
    mut commands: Commands,
    time: Res<Time>,
    mut ai_q: Query<(Entity, &Transform, &mut RushRoutine, Option<&mut TargetPosition>)>,
    player_q: Query<&Transform, With<PlayerControlled>>,
    mut ev: EventWriter<EventDistanceExited>
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let dt = time.delta();
    for (entity, transform, mut rush_routine, target_position) in ai_q.iter_mut() {
        if target_position.is_some() {
            continue;
        }
        if rush_routine.timer.tick(dt).just_finished() {
            rush_routine.timer.reset();
            if transform
                .translation
                .truncate()
                .distance_squared(player_transform.translation.truncate())
                > rush_routine.distance.powi(2)
            {
                ev.send(EventDistanceExited { parent: entity });
            }
            commands
                .entity(entity)
                .insert(TargetPosition::new(player_transform.translation.truncate()));
        }
    }
}

#[derive(Component)]
pub struct FollowRoutine {
    timer: Timer,
    distance: f32,
}

impl FollowRoutine {
    pub fn new(time: f32, distance: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
            distance,
        }
    }
}

fn update_follow_routine(
    mut commands: Commands,
    time: Res<Time>,
    mut ai_q: Query<(
        Entity,
        &Transform,
        &mut FollowRoutine,
        Option<&mut TargetPosition>,
    )>,
    player_q: Query<&Transform, With<PlayerControlled>>,
    mut ev: EventWriter<EventDistanceReached>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let dt = time.delta();
    for (entity, transform, mut follow_routine, target_position) in ai_q.iter_mut() {
        if transform
            .translation
            .truncate()
            .distance_squared(player_transform.translation.truncate())
            < follow_routine.distance.powi(2)
        {
            commands.entity(entity).remove::<TargetPosition>();
            ev.send(EventDistanceReached { parent: entity });
        }
        if !follow_routine.timer.tick(dt).just_finished() {
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
    }
}
