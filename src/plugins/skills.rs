use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use bevy::{prelude::*, math::vec2};
use bevy_rapier2d::prelude::{
    RigidBody,
    Collider,
    Velocity,
    Sensor, ActiveCollisionTypes, ActiveEvents
};

use crate::{
    GameState,
    GameResources,
    SPRITE_DRAW_SIZE,
    components::{
        Projectile,
        RotateAroundPoint,
        Slash, Attack, TTL, AttackCD, EquippedSkill
    },
    ATTACK_Z_INDEX,
    FIREBALL_SPEED,
    SLASH_SPEED, SPRITE_SIZE, PUNCH_SPEED
};

use super::events::{SkillEvent};

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
      app
        .add_systems((
            on_skill_used,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}



fn on_skill_used(
    game_resources: Res<GameResources>,
    mut commands: Commands,
    mut slash_events: EventReader<SkillEvent>,
) {

    for ev in slash_events.iter() {
        match ev.kind {
            EquippedSkill::Slash => {
                commands.entity(ev.parent).insert(AttackCD::new(1.0));

                let offset = ev.spawn_vector_norm * SPRITE_DRAW_SIZE * 0.7;
                let spawn_position = ev.start_position + offset;
                let mut attack_transform = Transform::from_xyz(spawn_position.x, spawn_position.y, ATTACK_Z_INDEX)
                    .with_rotation(Quat::from_rotation_z(ev.angle-FRAC_PI_4));
                attack_transform.rotate_around(ev.start_position.extend(ATTACK_Z_INDEX), Quat::from_rotation_z(-FRAC_PI_4));

                commands.spawn((
                    Attack {
                        value: 1.0,
                    },
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
                    RigidBody::Dynamic,
                    Sensor,
                    Collider::polyline(vec![
                        vec2(-SPRITE_DRAW_SIZE * 0.35, SPRITE_DRAW_SIZE * 0.35),
                        vec2(SPRITE_DRAW_SIZE * 0.2, SPRITE_DRAW_SIZE * 0.2),
                        vec2(SPRITE_DRAW_SIZE * 0.35, -SPRITE_DRAW_SIZE * 0.35)
                    ], None),
                    ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
                    ActiveEvents::COLLISION_EVENTS,
                ));
            },
            EquippedSkill::FireBall => {
                commands.entity(ev.parent).insert(AttackCD::new(2.0));

                let spawn_position = ev.start_position + ev.spawn_vector_norm * SPRITE_DRAW_SIZE;
                commands.spawn((
                    Attack {
                        value: 1.5,
                    },
                    TTL::new(0.7),
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
                    RigidBody::Dynamic,
                    Sensor,
                    Collider::cuboid(SPRITE_DRAW_SIZE / 2.0 - 10.0, SPRITE_DRAW_SIZE / 2.0 - 10.0),
                    ActiveEvents::COLLISION_EVENTS,
                    Velocity {
                        linvel: Vec2::new(ev.spawn_vector_norm.x * FIREBALL_SPEED, ev.spawn_vector_norm.y * FIREBALL_SPEED),
                        ..default()
                    },
                    Projectile
                ));
            },
            EquippedSkill::Punch(punch_offset) => {
                commands.entity(ev.parent).insert(AttackCD::new(0.1));
                commands.entity(ev.parent).remove::<EquippedSkill>();
                commands.entity(ev.parent).insert(EquippedSkill::Punch(-punch_offset));

                let spawn_position = ev.start_position +
                    ev.spawn_vector_norm * SPRITE_DRAW_SIZE * 0.8 +
                    vec2((ev.angle - FRAC_PI_2).cos(), (ev.angle - FRAC_PI_2).sin()) * punch_offset;
                commands.spawn((
                    Attack {
                        value: 0.5,
                    },
                    TTL::new(0.05),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                            rect: Some(Rect::new(7.0 * SPRITE_SIZE, 0.0, 8.0 * SPRITE_SIZE, SPRITE_SIZE)),
                            ..default()
                        },
                        texture: game_resources.image_handle.clone(),
                        transform: Transform::from_xyz(spawn_position.x, spawn_position.y, ATTACK_Z_INDEX)
                            .with_rotation(Quat::from_rotation_z(ev.angle)),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Sensor,
                    Collider::cuboid(SPRITE_DRAW_SIZE * 0.2, SPRITE_DRAW_SIZE * 0.2),
                    ActiveEvents::COLLISION_EVENTS,
                    Velocity {
                        linvel: Vec2::new(ev.spawn_vector_norm.x * PUNCH_SPEED, ev.spawn_vector_norm.y * PUNCH_SPEED),
                        ..default()
                    },
                    Projectile
                ));
            }
        }


    }
}
