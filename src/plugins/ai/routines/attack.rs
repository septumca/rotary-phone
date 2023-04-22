use std::f32::consts::FRAC_PI_2;

use crate::plugins::ai::events::EventAttackFinished;
use crate::plugins::ai::events::EventDistanceExited;
use crate::plugins::character::actor::ActorFacing;
use crate::plugins::character::actor::Weapon;
use crate::plugins::character::skills::ProjectileSpawnData;
use crate::plugins::character::skills::SkillUsedEvent;
use crate::ENEMY_PROJECTILE_FILTERS;
use crate::ENEMY_PROJECTILE_MEMBERSHIP;
use crate::PROJECTILE_SPEED;
use crate::{PlayerControlled, SPRITE_DRAW_SIZE};
use bevy::prelude::*;

enum AttackState {
    Start,
    Spawn,
    Finish,
}

pub enum AttackType {
    Fireball,
    Slash,
}

#[derive(Component)]
pub struct AttackRoutine {
    attack_type: AttackType,
    state: AttackState,
    timer_start: Timer,
    timer_finish: Timer,
    distance: f32,
}

impl AttackRoutine {
    pub fn new(
        attack_type: AttackType,
        start_seconds: f32,
        finish_seconds: f32,
        distance: f32,
    ) -> Self {
        Self {
            attack_type,
            state: AttackState::Start,
            timer_start: Timer::from_seconds(start_seconds, TimerMode::Once),
            timer_finish: Timer::from_seconds(finish_seconds, TimerMode::Once),
            distance,
        }
    }

    pub fn reset(&mut self) {
        self.state = AttackState::Start;
        self.timer_start.reset();
        self.timer_finish.reset();
    }
}

pub fn update_attack_routine(
    time: Res<Time>,
    mut ai_q: Query<(
        Entity,
        &ActorFacing,
        &Children,
        &Transform,
        &mut AttackRoutine,
    )>,
    player_q: Query<&Transform, With<PlayerControlled>>,
    mut weapon_q: Query<
        &mut Transform,
        (
            With<Weapon>,
            Without<PlayerControlled>,
            Without<AttackRoutine>,
        ),
    >,
    mut ev_skill: EventWriter<SkillUsedEvent>,
    mut ev_finished: EventWriter<EventAttackFinished>,
    mut ev_distance_exited: EventWriter<EventDistanceExited>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let dt = time.delta();
    for (entity, facing, children, transform, mut attack_routine) in ai_q.iter_mut() {
        let rotation_ratio = if facing.0 { 1.0 } else { -1.0 };
        let new_state = match attack_routine.state {
            AttackState::Start => {
                attack_routine.timer_start.tick(dt);
                for &child in children.iter() {
                    if let Ok(mut weapon_transform) = weapon_q.get_mut(child) {
                        let rotation =
                            attack_routine.timer_start.percent() * -FRAC_PI_2 * rotation_ratio;
                        weapon_transform.rotation = Quat::from_rotation_z(rotation);
                    }
                }
                if attack_routine.timer_start.just_finished() {
                    Some(AttackState::Spawn)
                } else {
                    None
                }
            }
            AttackState::Spawn => {
                let source_position = transform.translation.truncate();
                let velocity =
                    (player_transform.translation.truncate() - source_position).normalize();
                let angle = velocity.y.atan2(velocity.x);
                let position = source_position + velocity * SPRITE_DRAW_SIZE;
                let mut projectile_data = ProjectileSpawnData::new(
                    ENEMY_PROJECTILE_MEMBERSHIP,
                    ENEMY_PROJECTILE_FILTERS,
                    position,
                    velocity,
                    angle,
                );
                match attack_routine.attack_type {
                    AttackType::Fireball => {
                        projectile_data.multiply_velocity(PROJECTILE_SPEED * 0.6);
                        ev_skill.send(SkillUsedEvent::Fireball(projectile_data));
                    }
                    AttackType::Slash => {
                        projectile_data.multiply_velocity(PROJECTILE_SPEED);
                        ev_skill.send(SkillUsedEvent::Slash(projectile_data));
                    }
                };
                Some(AttackState::Finish)
            }
            AttackState::Finish => {
                attack_routine.timer_finish.tick(dt);
                for &child in children.iter() {
                    if let Ok(mut weapon_transform) = weapon_q.get_mut(child) {
                        let rotation = attack_routine.timer_finish.percent_left()
                            * -FRAC_PI_2
                            * rotation_ratio;
                        weapon_transform.rotation = Quat::from_rotation_z(rotation);
                    }
                }
                if attack_routine.timer_finish.just_finished() {
                    let distance = player_transform
                        .translation
                        .truncate()
                        .distance_squared(transform.translation.truncate());
                    if distance > attack_routine.distance.powi(2) {
                        ev_distance_exited.send(EventDistanceExited {
                            parent: entity,
                            distance: attack_routine.distance,
                        });
                    } else {
                        ev_finished.send(EventAttackFinished { parent: entity });
                    }
                }

                None
            }
        };

        if let Some(new_state) = new_state {
            attack_routine.state = new_state;
        }
    }
}
