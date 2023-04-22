use crate::plugins::ai::routines::attack::update_attack_routine;
use crate::plugins::ai::routines::rush::update_rush_routine;
use crate::plugins::ai::routines::follow::update_follow_routine;
use crate::GameState;
use bevy::prelude::*;

use super::AiExecutionSet;

pub mod attack;
pub mod follow;
pub mod rush;

pub struct RoutinesPlugin;

impl Plugin for RoutinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                update_follow_routine, 
                update_rush_routine,
                update_attack_routine,
            )
            .in_set(OnUpdate(GameState::Playing))
            .in_set(AiExecutionSet::Routines)
        );
    }
}
