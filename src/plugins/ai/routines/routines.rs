use crate::GameState;
use crate::plugins::ai::routines::update_rush_routine;
use crate::plugins::ai::routines::update_follow_routine;
use bevy::prelude::*;

pub struct RoutinesPlugin;

impl Plugin for RoutinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (update_follow_routine, update_rush_routine).in_set(OnUpdate(GameState::Playing)),
        );
    }
}


