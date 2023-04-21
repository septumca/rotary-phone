use crate::plugins::character::actor::TargetPosition;
use crate::plugins::character::dash::Dashing;
use crate::{GameState, PlayerControlled};
use bevy::prelude::*;

use super::events::{EventDistanceReached, EventDistanceExited};

pub struct RoutinesPlugin;

impl Plugin for RoutinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (update_follow_routine, update_rush_routine).in_set(OnUpdate(GameState::Playing)),
        );
    }
}


