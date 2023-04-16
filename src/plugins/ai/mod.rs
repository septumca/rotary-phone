use crate::plugins::ai::events::EventDistanceReached;
use crate::GameState;
use bevy::prelude::*;

use self::{routines::{RoutinesPlugin, FollowRoutine, RushRoutine}, events::{AiEventsPlugin, EventDistanceExited}};

use super::character::actor::TargetPosition;

pub mod routines;
pub mod events;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RoutinesPlugin);
        app.add_plugin(AiEventsPlugin);
        app.add_systems(
            (update_simple_ai,).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct SimpleAi;

fn update_simple_ai(
    mut commands: Commands,
    mut dr_ev: EventReader<EventDistanceReached>,
    mut de_ev: EventReader<EventDistanceExited>,
) {
    for ev in dr_ev.iter() {
        commands.entity(ev.parent).remove::<FollowRoutine>();
        commands.entity(ev.parent).remove::<TargetPosition>();
        commands.entity(ev.parent).insert(RushRoutine::new(1.0, 220.0)); 
    }

    for ev in de_ev.iter() {
        commands.entity(ev.parent).remove::<RushRoutine>();
        commands.entity(ev.parent).remove::<TargetPosition>();
        commands.entity(ev.parent).insert(FollowRoutine::new(0.1, 200.0)); 
    }
}

