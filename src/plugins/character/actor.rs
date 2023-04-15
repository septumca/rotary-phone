use crate::plugins::character::PlayerControlled;
use crate::plugins::character::Character;
use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, math::vec2};
use bevy_rapier2d::prelude::{
    Collider,
    RigidBody,
    KinematicCharacterController,
    ActiveEvents, Sensor, Velocity,
};

use crate::plugins::character::wiggle::WiggleEffect;
use crate::{GameState, components::{AttackCD, TTL, Attack}, SPRITE_DRAW_SIZE, SPRITE_SIZE, GameResources, ATTACK_Z_INDEX, PROJECTILE_SPEED};

pub const PLAYER_VELOCITY: f32 = 3.0;


pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                input,
                mouse_input,
            )
            .in_set(OnUpdate(GameState::Playing)));
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((
            move_to_target_position,
        ).in_set(OnUpdate(GameState::Playing)));
    }
}


#[derive(Component)]
pub struct TargetPosition(Vec2);

impl TargetPosition {
    pub fn new(pos: Vec2) -> Self {
        Self(pos)
    }

    pub fn update(&mut self, x: f32, y: f32) {
        self.0.x = x;
        self.0.y = y;
    }
}

fn move_to_target_position(
    mut commands: Commands,
    mut movable_q: Query<(Entity, &Character, &TargetPosition, &Transform, &mut KinematicCharacterController)>,
) {
    for (entity, character, target_position, transform, mut controller) in movable_q.iter_mut() {
        let delta_v = target_position.0 - transform.translation.truncate();
        if delta_v.length_squared() < 10.0 {
            commands.entity(entity).remove::<TargetPosition>();
            commands.entity(entity).remove::<WiggleEffect>();
            continue;
        }
        let velocity = delta_v.normalize() * character.speed;
        controller.translation = Some(velocity);
    }
}

fn input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(Entity, &Character, &mut KinematicCharacterController, Option<&WiggleEffect>), With<PlayerControlled>>,
) {
    let Ok((entity, character, mut controller, wiggle_effect)) = player_q.get_single_mut() else {
        return;
    };

    let mut velocity = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        velocity.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        velocity.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        velocity.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        velocity.x += 1.0;
    }

    if velocity != Vec2::ZERO {
        controller.translation = Some(velocity.normalize() * character.speed);
        if wiggle_effect.is_none() {
            commands.entity(entity).insert(WiggleEffect::default());
        }
    } else {
        if wiggle_effect.is_some() {
            commands.entity(entity).remove::<WiggleEffect>();
        }
    }
}

fn mouse_input(
    mut commands: Commands,
    game_resources: Res<GameResources>,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<(Entity, &Transform), (With<PlayerControlled>, Without<Camera>, Without<AttackCD>)>,
) {
    let Ok((entity, transform)) = player_q.get_single_mut() else {
        return;
    };
    let Ok(window) = window.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };
    let Some(mouse_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate()) else
    {
        return;
    };


    if mouse_button_input.pressed(MouseButton::Left) {
        let player_position = transform.translation.truncate();
        let spawn_vector = (mouse_position - player_position).normalize();
        let angle = spawn_vector.y.atan2(spawn_vector.x);
        let spawn_position = player_position + spawn_vector * SPRITE_DRAW_SIZE;
        commands.entity(entity).insert(AttackCD::new(1.0));
        
        commands.spawn((
            Attack {
                value: 0.3,
            },
            TTL::new(2.5),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(3.0 * SPRITE_SIZE, 0.0, 4.0 * SPRITE_SIZE, SPRITE_SIZE)),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: Transform::from_xyz(spawn_position.x, spawn_position.y, ATTACK_Z_INDEX).with_rotation(Quat::from_rotation_z(angle+FRAC_PI_2)),
                ..default()
            },
            RigidBody::Dynamic,
            Sensor,
            Collider::cuboid(SPRITE_DRAW_SIZE / 2.0 - 10.0, SPRITE_DRAW_SIZE / 2.0 - 10.0),
            ActiveEvents::COLLISION_EVENTS,
            Velocity {
                linvel: spawn_vector * PROJECTILE_SPEED,
                ..default()
            },
        ));
    }
}
