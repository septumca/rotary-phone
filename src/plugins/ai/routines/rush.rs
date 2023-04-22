use crate::plugins::ai::events::EventDistanceReached;
use crate::plugins::ai::EventDistanceExited;
use crate::plugins::character::actor::TargetPosition;
use crate::plugins::character::dash::Dashing;
use crate::PlayerControlled;
use bevy::math::vec2;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct RushRoutine {
    timer: Timer,
    min_distance: f32,
    max_distance: f32,
    offset: Option<Vec2>,
}

impl RushRoutine {
    pub fn new(time: f32, min_distance: f32, max_distance: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
            min_distance,
            max_distance,
            offset: None,
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = Some(offset);
        self
    }
}

pub fn update_rush_routine(
    mut commands: Commands,
    time: Res<Time>,
    mut ai_q: Query<(
        Entity,
        &Transform,
        &mut RushRoutine,
        Option<&mut TargetPosition>,
    )>,
    player_q: Query<&Transform, With<PlayerControlled>>,
    mut ev_de: EventWriter<EventDistanceExited>,
    mut ev_dr: EventWriter<EventDistanceReached>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let dt = time.delta();
    let mut rnd = rand::thread_rng();
    for (entity, transform, mut rush_routine, target_position) in ai_q.iter_mut() {
        let player_pos = player_transform.translation.truncate();
        let ai_pos = transform.translation.truncate();
        let distance_sq = ai_pos.distance_squared(player_pos);
        if target_position.is_some() {
            if distance_sq < rush_routine.min_distance.powi(2) {
                ev_dr.send(EventDistanceReached {
                    distance: rush_routine.min_distance,
                    parent: entity,
                });
            }
            continue;
        }
        if rush_routine.timer.tick(dt).just_finished() {
            rush_routine.timer.reset();
            if distance_sq > rush_routine.max_distance.powi(2) {
                ev_de.send(EventDistanceExited {
                    parent: entity,
                    distance: rush_routine.max_distance,
                });
            }
            let target_position = if let Some(offset) = rush_routine.offset {
                let ox = rnd.gen_range(-offset.x..offset.x);
                let oy = rnd.gen_range(-offset.y..offset.y);
                player_transform.translation.truncate() + vec2(ox, oy)
            } else {
                player_transform.translation.truncate()
            };
            commands
                .entity(entity)
                .insert(TargetPosition::new(target_position))
                .insert(Dashing::new(1.0));
        }
    }
}
