use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, window::WindowResolution, math::{vec2}};

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::{
    prelude::{
        RapierPhysicsPlugin,
        NoUserData,
        Collider,
        RigidBody,
        RapierConfiguration,
        KinematicCharacterController,
        ActiveCollisionTypes, LockedAxes, ActiveEvents,
    },
    render::RapierDebugRenderPlugin
};
use components::{Obstacle, PlayerControlled, EquippedSkill, Character, Health, HealthBar, Group};
use plugins::{timers::TimersPlugin, collision::CollisionPlugin, events::EventsPlugin, skills::SkillsPlugin, character::{CharacterPlugin, PLAYER_VELOCITY}, ai::{AiPlugin, steer::{SteerAi, SteerConfiguration, other::{RandomAroundArea}, avoid::{AvoidObstacles, AvoidGroup}, chase::{ChaseTargets, ChaseThenStrageAround}}}};

pub mod components;
pub mod plugins;
pub mod context_map;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const SPRITE_SIZE: f32 = 16.0;
const SCALE_FACTOR: f32 = 3.0;
const SPRITE_DRAW_SIZE: f32 = SPRITE_SIZE * SCALE_FACTOR;
const CHARACTER_Z_INDEX: f32 = 1.0;
const ATTACK_Z_INDEX: f32 = 1.5;
const FIREBALL_SPEED: f32 = 500.0;
const PUNCH_SPEED: f32 = 500.0;
const SLASH_SPEED: f32 = FRAC_PI_2;

pub fn lerp(start: f32, end: f32, ratio: f32) -> f32 {
    start * (1.0 - ratio) + end * ratio
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
}

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_state::<GameState>()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(EventsPlugin)
        .add_plugin(TimersPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(SkillsPlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(AiPlugin)
        .add_startup_system(setup)
        .add_system(setup_world.in_schedule(OnEnter(GameState::Playing)))
        ;

    #[cfg(debug_assertions)]
    {
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(DebugLinesPlugin::default());
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}


#[derive(Resource)]
pub struct GameResources {
    image_handle: Handle<Image>,
    font_handle: Handle<Font>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image_handle = asset_server.load("sprites.png");
    let font_handle = asset_server.load("QuinqueFive.ttf");

    commands.insert_resource(GameResources {
        image_handle,
        font_handle,
    });
    commands.spawn(Camera2dBundle::default());
}


fn setup_world(
    mut commands: Commands,
    game_resources: Res<GameResources>,
) {

    for (x, y) in [
        (-150., -100.),
        (150., -100.),
        (-150., 100.),
        (150., 100.),
    ] {
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(SPRITE_DRAW_SIZE / 2.0, SPRITE_DRAW_SIZE / 2.0),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(6.0 * SPRITE_SIZE, 0., 7.0 * SPRITE_SIZE, SPRITE_SIZE)),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: Transform::from_xyz(x, y, CHARACTER_Z_INDEX),
                ..default()
            },
            ActiveCollisionTypes::all(),
            Obstacle,
        ));
    }


    let player_entity = commands.spawn((
        Character {
            speed: PLAYER_VELOCITY
        },
        PlayerControlled,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                rect: Some(Rect::new(0.0 * SPRITE_SIZE, 0., 1.0 * SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            texture: game_resources.image_handle.clone(),
            transform: Transform::from_xyz(0., 0., CHARACTER_Z_INDEX),
            ..default()
        },
        EquippedSkill::Punch(SPRITE_DRAW_SIZE * 0.3),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(SPRITE_DRAW_SIZE * 0.3, SPRITE_DRAW_SIZE * 0.4),
        KinematicCharacterController::default(),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS
    )).id();

    commands.spawn((
        Character {
            speed: PLAYER_VELOCITY * 0.95
        },
        Health {
            act: 1.0,
            max: 1.0
        },
        Group(1),
        SteerAi::default(),
        // AvoidObstacles(SteerConfiguration::new(SPRITE_DRAW_SIZE, 200.0)),
        ChaseThenStrageAround(SteerConfiguration::new(200.0, 600.0), player_entity),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                rect: Some(Rect::new(8.0 * SPRITE_SIZE, 0., 9.0 * SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            texture: game_resources.image_handle.clone(),
            transform: Transform::from_xyz(200., 0., CHARACTER_Z_INDEX),
            ..default()
        },
        EquippedSkill::Punch(SPRITE_DRAW_SIZE * 0.3),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(SPRITE_DRAW_SIZE * 0.4, SPRITE_DRAW_SIZE * 0.4),
        KinematicCharacterController::default(),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS
    ))
    .with_children(|builder| {
        builder.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    color: Color::rgb(0.95, 0.25, 0.25),
                    custom_size: Some(Vec2::new(SPRITE_DRAW_SIZE, 8.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-SPRITE_DRAW_SIZE * 0.5, SPRITE_DRAW_SIZE * 0.6, 1.0)),
                ..default()
            },
            HealthBar
        ));
    });

    // commands.spawn((
    //     Character {
    //         speed: PLAYER_VELOCITY * 0.5
    //     },
    //     Health {
    //         act: 3.0,
    //         max: 3.0
    //     },
    //     SteerAi::new(1.0),
    //     AvoidObstacles(SteerConfiguration::new(SPRITE_DRAW_SIZE, 150.0)),
    //     AvoidGroup(SteerConfiguration::new(SPRITE_DRAW_SIZE, 150.0), 1),
    //     RandomAroundArea(vec2(-0., 0.), 100.),
    //     SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
    //             rect: Some(Rect::new(9.0 * SPRITE_SIZE, 0., 10.0 * SPRITE_SIZE, SPRITE_SIZE)),
    //             ..default()
    //         },
    //         texture: game_resources.image_handle.clone(),
    //         transform: Transform::from_xyz(-200., 0., CHARACTER_Z_INDEX),
    //         ..default()
    //     },
    //     EquippedSkill::Punch(SPRITE_DRAW_SIZE * 0.3),
    //     RigidBody::KinematicVelocityBased,
    //     Collider::cuboid(SPRITE_DRAW_SIZE * 0.4, SPRITE_DRAW_SIZE * 0.4),
    //     KinematicCharacterController::default(),
    //     LockedAxes::ROTATION_LOCKED,
    //     ActiveEvents::COLLISION_EVENTS
    // ))
    // .with_children(|builder| {
    //     builder.spawn((
    //         SpriteBundle {
    //             sprite: Sprite {
    //                 anchor: bevy::sprite::Anchor::BottomLeft,
    //                 color: Color::rgb(0.95, 0.25, 0.25),
    //                 custom_size: Some(Vec2::new(SPRITE_DRAW_SIZE, 8.0)),
    //                 ..default()
    //             },
    //             transform: Transform::from_translation(Vec3::new(-SPRITE_DRAW_SIZE * 0.5, SPRITE_DRAW_SIZE * 0.6, 1.0)),
    //             ..default()
    //         },
    //         HealthBar
    //     ));
    // });
}
