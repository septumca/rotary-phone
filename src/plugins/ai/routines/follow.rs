use crate::plugins::ai::EventDistanceReached;
use crate::plugins::character::actor::TargetPosition;
use crate::PlayerControlled;
use bevy::prelude::*;

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

pub fn update_follow_routine(
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
            ev.send(EventDistanceReached {
                parent: entity,
                distance: follow_routine.distance,
            });
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
