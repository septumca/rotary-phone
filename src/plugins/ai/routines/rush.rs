use crate::plugins::ai::EventDistanceExited;
use crate::plugins::character::actor::TargetPosition;
use crate::plugins::character::dash::Dashing;
use crate::PlayerControlled;
use bevy::prelude::*;


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

pub fn update_rush_routine(
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
                .insert(TargetPosition::new(player_transform.translation.truncate()))
                .insert(Dashing::new(1.0));
        }
    }
}

