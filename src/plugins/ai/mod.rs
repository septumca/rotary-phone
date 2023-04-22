use crate::plugins::ai::events::EventDistanceReached;
use crate::plugins::ai::routines::follow::FollowRoutine;
use crate::plugins::ai::routines::rush::RushRoutine;
use crate::GameState;
use bevy::{math::vec2, prelude::*};

use self::{
    events::{AiEventsPlugin, EventAttackFinished, EventDistanceExited},
    routines::{
        attack::{AttackRoutine, AttackType},
        RoutinesPlugin,
    },
};

use super::character::{
    actor::{ActorFacing, TargetPosition},
    dash::Dashing,
    PlayerControlled,
};

pub mod events;
pub mod routines;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AiExecutionSet {
    Initialisation,
    Routines,
    Controllers,
}

//TODO use state machine to remove/insert/update components
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RoutinesPlugin);
        app.add_plugin(AiEventsPlugin);
        app.add_systems(
            (initialize_simple_ai, initialize_shooting_ai)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(AiExecutionSet::Initialisation),
        );
        app.add_systems(
            (update_ai_facing, update_shooting_ai, update_simple_ai)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(AiExecutionSet::Controllers),
        );
        app.configure_sets(
            (
                AiExecutionSet::Initialisation,
                AiExecutionSet::Controllers,
                AiExecutionSet::Routines,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct ShootingAi;

fn update_ai_facing(
    player_q: Query<&Transform, With<PlayerControlled>>,
    mut ai_q: Query<(&Transform, &mut ActorFacing), Without<PlayerControlled>>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    for (transform, mut facing) in ai_q.iter_mut() {
        facing.0 = player_transform.translation.x < transform.translation.x;
    }
}

fn initialize_shooting_ai(
    mut commands: Commands,
    q: Query<
        Entity,
        (
            With<ShootingAi>,
            Without<FollowRoutine>,
            Without<AttackRoutine>,
        ),
    >,
) {
    for e in q.iter() {
        commands.entity(e).insert(FollowRoutine::new(0.1, 200.0));
    }
}

fn update_shooting_ai(
    mut commands: Commands,
    ai_q: Query<Entity, With<ShootingAi>>,
    mut q: Query<&mut AttackRoutine>,
    mut dr_ev: EventReader<EventDistanceReached>,
    mut de_ev: EventReader<EventDistanceExited>,
    mut af_ev: EventReader<EventAttackFinished>,
) {
    for ev in dr_ev.iter() {
        let Ok(_) = ai_q.get(ev.parent) else {
            continue;
        };
        commands.entity(ev.parent).remove::<FollowRoutine>();
        commands.entity(ev.parent).remove::<TargetPosition>();
        commands.entity(ev.parent).insert(AttackRoutine::new(
            AttackType::Fireball,
            0.8,
            0.4,
            220.0,
        ));
    }

    for ev in af_ev.iter() {
        let Ok(mut attack_routine) = q.get_mut(ev.parent) else {
            continue;
        };
        attack_routine.reset();
    }

    for ev in de_ev.iter() {
        let Ok(_) = ai_q.get(ev.parent) else {
            continue;
        };
        commands.entity(ev.parent).remove::<AttackRoutine>();
        commands
            .entity(ev.parent)
            .insert(FollowRoutine::new(0.1, 200.0));
    }
}

const ATTACK_DISTANCE: f32 = 80.0;
const RUSH_DISTANCE: f32 = 200.0;

#[derive(Component)]
pub struct SimpleAi;

fn initialize_simple_ai(
    mut commands: Commands,
    q: Query<
        Entity,
        (
            With<SimpleAi>,
            Without<AttackRoutine>,
            Without<FollowRoutine>,
            Without<RushRoutine>,
        ),
    >,
) {
    for e in q.iter() {
        commands
            .entity(e)
            .insert(FollowRoutine::new(0.1, RUSH_DISTANCE));
    }
}

fn update_simple_ai(
    mut commands: Commands,
    ai_q: Query<Entity, With<SimpleAi>>,
    mut q: Query<&mut AttackRoutine>,
    mut dr_ev: EventReader<EventDistanceReached>,
    mut de_ev: EventReader<EventDistanceExited>,
    mut af_ev: EventReader<EventAttackFinished>,
) {
    for ev in dr_ev.iter() {
        let Ok(_) = ai_q.get(ev.parent) else {
            continue;
        };
        if ev.distance <= ATTACK_DISTANCE {
            commands.entity(ev.parent).remove::<RushRoutine>();
            commands.entity(ev.parent).remove::<Dashing>();
            commands.entity(ev.parent).remove::<FollowRoutine>();
            commands.entity(ev.parent).remove::<TargetPosition>();
            commands.entity(ev.parent).insert(AttackRoutine::new(
                AttackType::Slash,
                0.2,
                0.1,
                ATTACK_DISTANCE,
            ));
        } else if ev.distance <= RUSH_DISTANCE {
            commands.entity(ev.parent).remove::<FollowRoutine>();
            commands.entity(ev.parent).remove::<Dashing>();
            commands.entity(ev.parent).remove::<TargetPosition>();
            commands.entity(ev.parent).remove::<AttackRoutine>();
            commands.entity(ev.parent).insert(
                RushRoutine::new(0.5, ATTACK_DISTANCE, RUSH_DISTANCE).with_offset(vec2(50.0, 50.0)),
            );
        }
    }

    for ev in af_ev.iter() {
        let Ok(mut attack_routine) = q.get_mut(ev.parent) else {
            continue;
        };
        attack_routine.reset();
    }

    for ev in de_ev.iter() {
        let Ok(_) = ai_q.get(ev.parent) else {
            continue;
        };
        if ev.distance >= RUSH_DISTANCE {
            commands.entity(ev.parent).remove::<RushRoutine>();
            commands.entity(ev.parent).remove::<Dashing>();
            commands.entity(ev.parent).remove::<AttackRoutine>();
            commands.entity(ev.parent).remove::<TargetPosition>();
            commands
                .entity(ev.parent)
                .insert(FollowRoutine::new(0.1, RUSH_DISTANCE));
        } else if ev.distance >= ATTACK_DISTANCE {
            commands.entity(ev.parent).remove::<FollowRoutine>();
            commands.entity(ev.parent).remove::<Dashing>();
            commands.entity(ev.parent).remove::<TargetPosition>();
            commands.entity(ev.parent).remove::<AttackRoutine>();
            commands.entity(ev.parent).insert(
                RushRoutine::new(0.5, ATTACK_DISTANCE, RUSH_DISTANCE).with_offset(vec2(50.0, 50.0)),
            );
        }
    }
}
