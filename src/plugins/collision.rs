use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{GameState, components::{Attack, Wall}};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                handle_events,
            ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn handle_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    // mut contact_force_events: EventReader<ContactForceEvent>,
    attack_q: Query<Entity, With<Attack>>,
    wall_q: Query<Entity, With<Wall>>,
) {
    for collision_event in collision_events.iter() {
        // info!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Some((attack_e, _wall_e)) = if attack_q.get(*e1).is_ok() && wall_q.get(*e2).is_ok() {
                    Some((*e1, *e2))
                } else if attack_q.get(*e2).is_ok() && wall_q.get(*e1).is_ok() {
                    Some((*e2, *e1))
                } else {
                    None
                } {
                    info!("REMOVING ATTACK");
                    commands.entity(attack_e).despawn_recursive();
                }
            },
            _ => {}
        }
    }

    // for contact_force_event in contact_force_events.iter() {
    //     println!("Received contact force event: {:?}", contact_force_event);
    // }
}
