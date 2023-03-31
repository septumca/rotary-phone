use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use bevy::{prelude::*, math::vec2};
use bevy_rapier2d::prelude::{
    RigidBody,
    Collider,
    ActiveCollisionTypes,
    ActiveEvents,
    Velocity,
    Sensor
};

use crate::{
    GameState,
    GameResources,
    SPRITE_DRAW_SIZE,
    components::{
        Attack,
        TTL,
        Projectile,
        RotateAroundPoint,
        Slash
    },
    SPRITE_SIZE,
    ATTACK_Z_INDEX,
    FIREBALL_SPEED,
    SLASH_SPEED
};

use super::events::{FireEvent, SlashEvent};

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
      app
        .add_systems((
            on_fire,
            on_slash,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn on_slash(
    game_resources: Res<GameResources>,
    mut commands: Commands,
    mut slash_events: EventReader<SlashEvent>,
) {

    for ev in slash_events.iter() {
        let offset = ev.spawn_vector_norm * SPRITE_DRAW_SIZE * 0.7;
        let spawn_position = ev.start_position + offset;
        let mut attack_transform = Transform::from_xyz(spawn_position.x, spawn_position.y, ATTACK_Z_INDEX)
            .with_rotation(Quat::from_rotation_z(ev.angle-FRAC_PI_4));
        attack_transform.rotate_around(ev.start_position.extend(ATTACK_Z_INDEX), Quat::from_rotation_z(-FRAC_PI_4));

        commands.spawn((
            Attack,
            RotateAroundPoint::new(ev.start_position.extend(ATTACK_Z_INDEX), SLASH_SPEED),
            Slash,
            TTL::new(0.2),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(5.0 * SPRITE_SIZE, 0.0, 6.0 * SPRITE_SIZE, SPRITE_SIZE)),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: attack_transform,
                ..default()
            },
            Sensor,
            Collider::polyline(vec![
                vec2(-SPRITE_DRAW_SIZE * 0.35, SPRITE_DRAW_SIZE * 0.35),
                vec2(SPRITE_DRAW_SIZE * 0.2, SPRITE_DRAW_SIZE * 0.2),
                vec2(SPRITE_DRAW_SIZE * 0.35, -SPRITE_DRAW_SIZE * 0.35)
            ], None),
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

fn on_fire(
    game_resources: Res<GameResources>,
    mut commands: Commands,
    mut fire_events: EventReader<FireEvent>,
) {
    for ev in fire_events.iter() {
        let spawn_position = ev.start_position + ev.spawn_vector_norm * SPRITE_DRAW_SIZE;
        commands.spawn((
            Attack,
            TTL::new(2.0),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(3.0 * SPRITE_SIZE, 0.0, 4.0 * SPRITE_SIZE, SPRITE_SIZE)),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: Transform::from_xyz(spawn_position.x, spawn_position.y, ATTACK_Z_INDEX).with_rotation(Quat::from_rotation_z(ev.angle+FRAC_PI_2)),
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            Collider::cuboid(SPRITE_DRAW_SIZE / 2.0 - 10.0, SPRITE_DRAW_SIZE / 2.0 - 10.0),
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
            ActiveEvents::COLLISION_EVENTS,
            Velocity {
                linvel: Vec2::new(ev.spawn_vector_norm.x * FIREBALL_SPEED, ev.spawn_vector_norm.y * FIREBALL_SPEED),
                ..default()
            },
            Projectile
        ));
    }
}