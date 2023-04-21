use crate::plugins::character::Character;
use crate::plugins::character::PlayerControlled;

use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

use crate::plugins::character::wiggle::WiggleEffect;
use crate::{components::AttackCD, GameState, PROJECTILE_SPEED, SPRITE_DRAW_SIZE};

use super::skills::FireballSpawnData;
use super::skills::SkillUsedEvent;

pub const PLAYER_VELOCITY: f32 = 3.0;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((input, mouse_input).in_set(OnUpdate(GameState::Playing)));
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                move_to_target_position,
                update_facing,
                add_move_effect::<WiggleEffect>
                    .after(move_to_target_position)
                    .after(input),
                remove_move_effect::<WiggleEffect>
                    .after(move_to_target_position)
                    .after(input),
            )
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct ActorFacing(pub bool);

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct WeaponSprite;

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

fn add_move_effect<T: Component + Default>(
    mut commands: Commands,
    mut q: Query<(Entity, &KinematicCharacterController), Without<T>>,
) {
    for (entity, controller) in q.iter_mut() {
        if controller.translation.is_some() {
            commands.entity(entity).insert(T::default());
        }
    }
}

fn remove_move_effect<T: Component>(
    mut commands: Commands,
    mut q: Query<(Entity, &KinematicCharacterController), With<T>>,
) {
    for (entity, controller) in q.iter_mut() {
        if controller.translation.is_none() {
            commands.entity(entity).remove::<T>();
        }
    }
}

fn update_facing(
    mut actor_q: Query<
        (&ActorFacing, &mut Sprite, &Children),
        (Without<WeaponSprite>, Without<Weapon>, Changed<ActorFacing>),
    >,
    weapon_q: Query<&Children, With<Weapon>>,
    mut weapon_sprite_q: Query<
        (&mut Sprite, &mut Transform),
        (With<WeaponSprite>, Without<Weapon>),
    >,
) {
    for (actor_facing, mut sprite, children) in actor_q.iter_mut() {
        sprite.flip_x = actor_facing.0;
        for &children_weapon in children {
            let Ok(weapon_children) = weapon_q.get(children_weapon) else {
                continue;
            };
            for &weapon_child in weapon_children {
                let Ok((mut weapon_sprite, mut transform)) =
                    weapon_sprite_q.get_mut(weapon_child) else 
                {
                    continue;
                };
                //weapon_sprite.flip_x = actor_facing.0;
                if actor_facing.0 {
                    transform.translation.x = -SPRITE_DRAW_SIZE * 0.5;
                } else {
                    transform.translation.x = SPRITE_DRAW_SIZE * 0.5;
                }
            }
        }
    }
}

fn move_to_target_position(
    mut commands: Commands,
    mut movable_q: Query<(
        Entity,
        &Character,
        &TargetPosition,
        &Transform,
        &mut KinematicCharacterController,
    )>,
) {
    for (entity, character, target_position, transform, mut controller) in movable_q.iter_mut() {
        let delta_v = target_position.0 - transform.translation.truncate();
        if delta_v.length_squared() < 10.0 {
            commands.entity(entity).remove::<TargetPosition>();
            continue;
        }
        let velocity = delta_v.normalize() * character.speed;
        controller.translation = Some(velocity);
    }
}

fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(&Character, &mut KinematicCharacterController), With<PlayerControlled>>,
) {
    let Ok((character, mut controller)) = player_q.get_single_mut() else {
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
    }
}

fn mouse_input(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<
        (Entity, &Transform),
        (With<PlayerControlled>, Without<Camera>, Without<AttackCD>),
    >,
    mut ev: EventWriter<SkillUsedEvent>,
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
        ev.send(SkillUsedEvent::Fireball(FireballSpawnData::new(
            spawn_position,
            spawn_vector * PROJECTILE_SPEED,
            angle,
        )));
    }
}
