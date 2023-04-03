use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{GameState, components::{Attack, Wall, Health}};

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
    attack_q: Query<&Attack>,
    wall_q: Query<Entity, With<Wall>>,
    mut health_q: Query<&mut Health>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let mut attack_e = *e1;
                let mut other = *e2;
                let Ok(attack) = attack_q.get(attack_e).or_else(|_| {
                    other = *e1;
                    attack_e = *e2;
                    attack_q.get(attack_e)
                }) else {
                    continue;
                };
                if let Ok(_) =  wall_q.get(other) {
                    info!("WALL HIT");
                }

                if let Ok(mut health) =  health_q.get_mut(other) {
                    health.act -= attack.value;
                    info!("HIT someone with health, current health is {}", health.act);
                }
                commands.entity(attack_e).despawn_recursive();
            },
            _ => {}
        }
    }

    // for contact_force_event in contact_force_events.iter() {
    //     println!("Received contact force event: {:?}", contact_force_event);
    // }
}
