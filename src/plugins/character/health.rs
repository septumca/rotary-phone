use bevy::prelude::*;

use crate::{SPRITE_DRAW_SIZE, GameState};

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct Health {
    max: f32,
    act: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { max, act: max }
    }
    
    pub fn change(&mut self, change: f32) {
        self.act += change;
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            cleanup_on_zero_health,
            update_health_bar,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn cleanup_on_zero_health(
    mut commands: Commands,
    health_q: Query<(Entity, &Health), Changed<Health>>,
) {
    for (entity, health) in health_q.iter() {
        if health.act <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_health_bar(
    health_q: Query<&Health, Changed<Health>>,
    mut healthbar_q: Query<(&Parent, &mut Sprite), (With<HealthBar>, Without<Health>)>,
) {
    for (parent, mut sprite) in healthbar_q.iter_mut() {
        let Ok(health) = health_q.get(parent.get()) else {
            continue;
        };
        let ratio = health.act / health.max;
        sprite.custom_size = Some(Vec2::new(SPRITE_DRAW_SIZE * ratio, 8.0));
    }
}

