use std::f32::consts::FRAC_PI_2;

use crate::GameResources;
use crate::components::Attack;
use crate::components::TTL;
use crate::plugins::character::flip::FlipEffect;
use crate::GameState;
use crate::ATTACK_Z_INDEX;
use crate::SPRITE_DRAW_SIZE;
use crate::SPRITE_SIZE;
use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::prelude::Sensor;
use bevy_rapier2d::prelude::Velocity;

pub struct FireballSpawnData {
    position: Vec2,
    velocity: Vec2,
    angle: f32,
}

impl FireballSpawnData {
    pub fn new(position: Vec2, velocity: Vec2, angle: f32) -> Self {
       Self { position, velocity, angle } 
    }
}

pub enum SkillUsedEvent {
    Fireball(FireballSpawnData),
}

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SkillUsedEvent>()
            .add_systems((on_skill_used,).in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn on_skill_used(
    game_resources: Res<GameResources>,
    mut commands: Commands,
    mut ev: EventReader<SkillUsedEvent>
) {
    for e in ev.iter() {
        match e {
            SkillUsedEvent::Fireball(data) => {
                commands.spawn((
                    FlipEffect::new(),
                    Attack { value: 0.3 },
                    TTL::new(2.5),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                            rect: Some(Rect::new(
                                3.0 * SPRITE_SIZE,
                                0.0,
                                4.0 * SPRITE_SIZE,
                                SPRITE_SIZE,
                            )),
                            ..default()
                        },
                        texture: game_resources.image_handle.clone(),
                        transform: Transform::from_xyz(
                            data.position.x,
                            data.position.y,
                            ATTACK_Z_INDEX,
                        )
                        .with_rotation(Quat::from_rotation_z(data.angle + FRAC_PI_2)),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Sensor,
                    Collider::cuboid(SPRITE_DRAW_SIZE / 2.0 - 10.0, SPRITE_DRAW_SIZE / 2.0 - 10.0),
                    ActiveEvents::COLLISION_EVENTS,
                    Velocity {
                        linvel: data.velocity,
                        ..default()
                    },
                ));
            }
        }
    }
}
