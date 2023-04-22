use crate::GameState;
use crate::Obstacle;
use crate::CHARACTER_Z_INDEX;
use crate::OBSTACLE_FILTERS;
use crate::OBSTACLE_MEMBERSHIP;
use crate::SPRITE_SIZE;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::ActiveEvents;
use bevy_rapier2d::prelude::CollisionGroups;
use bevy_rapier2d::prelude::Group;
use bevy_rapier2d::prelude::{ActiveCollisionTypes, Collider, RigidBody};

use crate::{GameResources, SPRITE_DRAW_SIZE};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_arena.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn create_arena(
    mut commands: Commands,
    game_resources: Res<GameResources>,
    window: Query<&Window>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let tiles_width = window.width() / SPRITE_DRAW_SIZE;
    let tiles_height = window.height() / SPRITE_DRAW_SIZE;

    for y in [
        window.height() / 2.0 - SPRITE_DRAW_SIZE / 2.0,
        -window.height() / 2.0 + SPRITE_DRAW_SIZE / 2.0,
    ] {
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, y, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(window.width() / 2.0, SPRITE_DRAW_SIZE / 2.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(
                Group::from_bits_truncate(OBSTACLE_MEMBERSHIP),
                Group::from_bits_truncate(OBSTACLE_FILTERS),
            ),
            Obstacle,
        ));
        for i in 0..tiles_width as usize {
            let x = -window.width() / 2.0 + ((i as f32 + 0.5) * SPRITE_DRAW_SIZE);
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(
                        6.0 * SPRITE_SIZE,
                        0.,
                        7.0 * SPRITE_SIZE,
                        SPRITE_SIZE,
                    )),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: Transform::from_xyz(x, y, CHARACTER_Z_INDEX),
                ..default()
            },));
        }
    }

    for x in [
        window.width() / 2.0 - SPRITE_DRAW_SIZE / 2.0,
        -window.width() / 2.0 + SPRITE_DRAW_SIZE / 2.0,
    ] {
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(x, 0.0, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(SPRITE_DRAW_SIZE / 2.0, window.height() / 2.0),
            //ActiveCollisionTypes::all(),
            CollisionGroups::new(
                Group::from_bits_truncate(OBSTACLE_MEMBERSHIP),
                Group::from_bits_truncate(OBSTACLE_FILTERS),
            ),
            Obstacle,
        ));
        for i in 1..(tiles_height - 1.0) as usize {
            let y = -window.height() / 2.0 + ((i as f32 + 0.5) * SPRITE_DRAW_SIZE);
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(
                        6.0 * SPRITE_SIZE,
                        0.,
                        7.0 * SPRITE_SIZE,
                        SPRITE_SIZE,
                    )),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: Transform::from_xyz(x, y, CHARACTER_Z_INDEX),
                ..default()
            },));
        }
    }
}
