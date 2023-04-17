use bevy::{prelude::*, math::{vec2, vec3}};
use bevy_rapier2d::prelude::KinematicCharacterController;

use crate::{components::TTL, GameState, GameResources, SPRITE_DRAW_SIZE};

use super::{actor::TargetPosition, PlayerControlled};

const DASH_ALPHA_RATIO: f32 = 0.7;

pub struct DashEffectPlugin;

impl Plugin for DashEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                update_dash_effect, 
                update_dashing_entity, 
                remove_dash_effect,
            ).in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct DashEffect;

#[derive(Component)]
pub struct Dashing {
    timer: Timer,
}

impl Dashing {
    pub fn new(time: f32) -> Self {
        Self {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
        }
    }
}

fn remove_dash_effect(
    mut commands: Commands,
    ai_q: Query<Entity, (With<Dashing>, Without<TargetPosition>)>,
    player_q: Query<(Entity, &KinematicCharacterController), (With<PlayerControlled>, With<Dashing>)>,
) {
    for entity in ai_q.iter() {
        commands.entity(entity).remove::<Dashing>();
    }

    for (entity, controller) in player_q.iter() {
        if controller.translation.is_none() {
            commands.entity(entity).remove::<Dashing>();
        }
    }
}

fn update_dash_effect(
    mut q: Query<(&mut Sprite, &TTL), With<DashEffect>>
) {
    for (mut sprite, ttl) in q.iter_mut() {
        sprite.color.set_a(DASH_ALPHA_RATIO - ttl.0.percent() * DASH_ALPHA_RATIO);
    }
}

fn update_dashing_entity(
    game_resources: Res<GameResources>,
    mut commands: Commands,
    timer: Res<Time>,
    mut q: Query<(&Sprite, &Transform, &mut Dashing)>,
) {
    let dt = timer.delta();
    for (sprite, transform, mut dashing) in q.iter_mut() {
        let translation = vec3(
            transform.translation.x,
            transform.translation.y,
            transform.translation.z - 0.1
        );
        if dashing.timer.tick(dt).just_finished() {
            commands.spawn((
                DashEffect,
                TTL::new(0.3),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                        rect: sprite.rect.clone(), 
                        ..default()
                    },
                    texture: game_resources.image_handle.clone(),
                    transform: transform.with_translation(translation),
                    ..default()
                },
            ));
        }
    }
}
